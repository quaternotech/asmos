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
use x86_64::structures::paging::{OffsetPageTable, Page, PageTable, PageTableFlags, Size2MiB};
use x86_64::structures::paging::{PageSize, Translate};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRange;

use crate::arch::memory::physical::total_memory_aligned;

use super::meta;

mod physical;

// Todo:
// 1. Physical memory manager: Find and return frames. (partially done)
// 2. Virtual memory manager: Map those frames into the virtual space.
// 3. Linear memory manager: Interface for performing dynamic allocations via heap.
pub fn init(memory_map_tag: &'static MemoryMapTag) -> Result<(), MapToError<Size2MiB>> {
    physical::init(&memory_map_tag).ok();

    // map_physical_memory(memory_map_tag)?;

    // let mut mapper = unsafe { get_mapper(meta::physical_memory_offset()) };

    // map_reserved_region(&mut mapper).ok();

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

// fn map_physical_memory(memory_map_tag: &'static MemoryMapTag) -> Result<(), MapToError<Size2MiB>> {
//     // Temporarily use kernel offset.
//     let mut mapper = unsafe { get_mapper(meta::kernel_offset()) };
//     // Temporarily map some pages at the end of the kernel.
//     let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;
//     let temp_map_size = 0x400000;
//     unsafe {
//         // todo: fix: these figures are very brittle.
//         physical::map_range(&mut mapper, flags, meta::kernel_end() + 0x400000, meta::kernel_end() + 0x400000 + meta::kernel_offset(), temp_map_size)?;
//     }
//
//     // Map entire available physical memory at an offset.
//     let total_available_memory = total_memory_aligned::<Size2MiB>(memory_map_tag);
//     unsafe {
//         physical::map_range(&mut mapper, flags, 0, meta::physical_memory_offset(), total_available_memory)?;
//     }
//
//     // Unmap the temporarily mapped pages after the kernel.
//     let page_range = get_page_range::<Size2MiB>(meta::kernel_end() + 0x400000 + meta::kernel_offset(), temp_map_size);
//     unsafe {
//         physical::unmap_range(&mut mapper, page_range).ok();
//     }
//
//     Ok(())
// }
//
// fn map_reserved_region(mapper: &mut impl Mapper<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
//     let frame_range = get_frame_range(0, meta::reserved_region_size());
//     let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
//     unsafe {
//         physical::identity_map_range(mapper, flags, frame_range)?;
//     }
//
//     Ok(())
// }
