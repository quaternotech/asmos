// Copyright 2023 Quaterno LLC
//
// Author: Mansoor Ahmed Memon <mansoorahmed.one@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use multiboot2::MemoryMapTag;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size2MiB, Size4KiB, Translate};
use x86_64::structures::paging::{Mapper, PageSize};
use x86_64::structures::paging::frame::PhysFrameRange;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRange;

use super::meta;

mod pmm;

pub fn init(memory_map_tag: &'static MemoryMapTag) -> Result<(), MapToError<Size2MiB>> {
    unsafe { pmm::init(memory_map_tag) }.expect("kernel failed to initialize physical memory manager");

    // We haven't mapped physical memory to an offset yet. So, we use the kernel offset for now.
    // I need to replace this kernel offset with physical memory offset in order to make it work.
    // This means that, I must allocate physical memory and use that mapper table.
    let mut mapper = unsafe { get_mapper(meta::kernel_offset()) };

    let kernel_size = meta::kernel_size();

    {
        let mut frame_allocator = pmm::PHYSICAL_MEMORY_MANAGER.lock();
        let frame_allocator = frame_allocator.as_mut().unwrap();

        // Todo: This is a temporary fix. This needs to be handled properly in the page fault handler.
        let phys_frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(kernel_size));
        let page = Page::<Size2MiB>::containing_address(VirtAddr::new(kernel_size + meta::kernel_offset()));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;
        unsafe {
            mapper.map_to(page, phys_frame, flags, frame_allocator)?.flush();
        }
    }

    map_reserved_region(&mut mapper).ok();
    map_physical_memory(memory_map_tag, &mut mapper)?;

    let mut mapper = unsafe { get_mapper(meta::physical_memory_offset()) };

    allocate_heap(&mut mapper).ok();

    Ok(())
}

unsafe fn active_pt4(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let pt4_frame = Cr3::read().0;

    let physical_addr = pt4_frame.start_address();
    let virtual_addr = VirtAddr::new(physical_memory_offset.as_u64() + physical_addr.as_u64());
    let page_table_ptr: *mut PageTable = virtual_addr.as_mut_ptr();

    &mut *page_table_ptr
}

unsafe fn get_mapper(physical_memory_offset: u64) -> OffsetPageTable<'static> {
    let physical_memory_offset = VirtAddr::new(physical_memory_offset);
    let pt4 = active_pt4(physical_memory_offset);
    OffsetPageTable::new(pt4, physical_memory_offset)
}

pub fn v2p(vaddr: VirtAddr) -> Option<PhysAddr> {
    let mapper = unsafe { get_mapper(meta::physical_memory_offset()) };
    mapper.translate_addr(vaddr)
}

pub fn p2v(paddr: PhysAddr) -> VirtAddr {
    VirtAddr::new(paddr.as_u64()) + meta::physical_memory_offset()
}

pub fn total_memory(memory_map_tag: &'static MemoryMapTag) -> u64 {
    memory_map_tag.available_memory_areas().map(|area| area.size()).sum::<u64>()
}

pub fn total_memory_aligned<S: PageSize>(memory_map_tag: &'static MemoryMapTag) -> u64 {
    x86_64::align_up(total_memory(memory_map_tag), S::SIZE)
}

fn get_page_range<S: PageSize>(begin: u64, size: u64) -> PageRange<S> {
    let begin = VirtAddr::new(begin);
    let end = begin + size;
    let first = Page::<S>::containing_address(begin);
    let last = Page::<S>::containing_address(end);
    Page::<S>::range(first, last)
}

fn get_frame_range<S: PageSize>(begin: u64, size: u64) -> PhysFrameRange<S> {
    let begin = PhysAddr::new(begin);
    let end = begin + size;
    let first = PhysFrame::<S>::containing_address(begin);
    let last = PhysFrame::<S>::containing_address(end);
    PhysFrame::<S>::range(first, last)
}

fn map_reserved_region(mapper: &mut impl Mapper<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let frame_range = get_frame_range(0, meta::reserved_region_size());
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
        pmm::identity_map_range(mapper, flags, frame_range)?;
    }

    Ok(())
}

fn map_physical_memory(memory_map_tag: &'static MemoryMapTag,
                       mapper: &mut impl Mapper<Size2MiB>) -> Result<(), MapToError<Size2MiB>> {
    let total_available_memory = total_memory_aligned::<Size2MiB>(memory_map_tag);
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;
    unsafe {
        pmm::map_range(mapper, flags, 0, meta::physical_memory_offset(), total_available_memory)?;
    }

    Ok(())
}

fn allocate_heap(mapper: &mut impl Mapper<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let page_range = get_page_range(meta::heap_begin(), meta::heap_size());
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
        pmm::allocate_range(mapper, flags, page_range)?;
    }

    Ok(())
}
