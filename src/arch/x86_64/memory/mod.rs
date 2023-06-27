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

use linked_list_allocator::Heap;
use multiboot2::MemoryMapTag;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{Mapper, PageSize, Translate};
use x86_64::structures::paging::{OffsetPageTable, Page, PageTable, PageTableFlags, Size2MiB, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRange;

use phys::get_frame_range;

use super::meta;

mod phys;
mod virt;

pub fn init(memory_map_tag: &'static MemoryMapTag) -> Result<(), ()> {
    phys::init(memory_map_tag)?;

    map_physical_memory().ok();

    virt::init()?;

    let mut mapper = unsafe { get_mapper(meta::physical_memory_offset()) };

    map_reserved_region(&mut mapper).ok();

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

fn get_page_range<S: PageSize>(begin: u64, size: u64) -> PageRange<S> {
    let begin = VirtAddr::new(begin);
    let end = begin + size;
    let first = Page::<S>::containing_address(begin);
    let last = Page::<S>::containing_address(end);
    Page::<S>::range(first, last)
}

fn map_physical_memory() -> Result<(), MapToError<Size2MiB>> {
    // Temporarily use kernel offset.
    let mut mapper = unsafe { get_mapper(meta::kernel_offset()) };
    // Temporarily map some pages at the end of the kernel.
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;
    let temp_map_size = 0x400000;
    let offset = 0x200000;
    unsafe {
        // todo: fix: these figures are very brittle.
        virt::map_range(&mut mapper, flags, meta::kernel_end() + offset, meta::kernel_end() + meta::kernel_offset() + offset, temp_map_size)?;
    }

    // Map entire available phys memory at an offset.
    let total_available_memory = total_memory_aligned::<Size2MiB>();
    unsafe {
        virt::map_range(&mut mapper, flags, 0, meta::physical_memory_offset(), total_available_memory)?;
    }

    // Unmap the temporarily mapped pages after the kernel.
    let page_range = get_page_range::<Size2MiB>(meta::kernel_end() + meta::kernel_offset() + offset, temp_map_size);
    unsafe {
        virt::unmap_range(&mut mapper, page_range).ok();
    }

    Ok(())
}

fn map_reserved_region(mapper: &mut impl Mapper<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let frame_range = get_frame_range(0, meta::reserved_region_size());
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
        virt::identity_map_range(mapper, flags, frame_range)?;
    }

    Ok(())
}

fn allocate_heap(mapper: &mut impl Mapper<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    let page_range = get_page_range(meta::heap_begin(), meta::heap_size());
    unsafe {
        virt::allocate_range(mapper, flags, page_range)?;
    }
    Ok(())
}


pub fn total_memory_aligned<S: PageSize>() -> u64 {
    x86_64::align_up(meta::total_memory().unwrap(), S::SIZE)
}
