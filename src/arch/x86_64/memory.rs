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

use core::iter::zip;

use multiboot2::MemoryMapTag;
use x86_64::{PhysAddr, registers, VirtAddr};
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size2MiB, Size4KiB};
use x86_64::structures::paging::{FrameAllocator, Mapper, PageSize};
use x86_64::structures::paging::mapper::MapToError;

use crate::arch::x86_64::meta;
use crate::arch::x86_64::meta::{kernel_end, kernel_offset};
use crate::arch::x86_64::preliminary::stack::{KERNEL_STACK, KERNEL_STACK_SIZE, Stack};
use crate::serial_println;

use super::preliminary::configurations::CONFIG_CORE_MEMORY_STACK_SIZE;

pub const PHYSICAL_MEMORY_OFFSET: u64 = 0xFFFF_8000_0000_0000;
pub const HEAP_START: u64 = 0xFFFF_C000_0000_0000;
pub const HEAP_SIZE: u64 = 0x200000;
pub const RELOCATED_STACK: u64 = 0xFFFF_FF80_0000_0000;

pub struct AreaFrameAllocator {
    memory_map_tag: &'static MemoryMapTag,
    next: usize,
}

impl AreaFrameAllocator {
    pub unsafe fn new(memory_map_tag: &'static MemoryMapTag) -> Self {
        AreaFrameAllocator {
            memory_map_tag,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item=PhysFrame> {
        // Get usable regions from memory map.
        let regions = self.memory_map_tag.available_memory_areas();
        // Map each region to its address range.
        let addr_ranges = regions.map(|r| r.start_address()..r.end_address());
        // Transform to an iterator of frame start addresses.
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(Size4KiB::SIZE as usize));
        // Create `PhysFrame` types from the start addresses.
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

pub unsafe fn active_pt4(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let pt4_frame = Cr3::read().0;

    let physical_addr = pt4_frame.start_address();
    let virtual_addr = VirtAddr::new(physical_memory_offset.as_u64() + physical_addr.as_u64());
    let page_table_ptr: *mut PageTable = virtual_addr.as_mut_ptr();

    &mut *page_table_ptr
}

pub unsafe fn mapper(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let pt4 = active_pt4(physical_memory_offset);
    OffsetPageTable::new(pt4, physical_memory_offset)
}

unsafe impl FrameAllocator<Size4KiB> for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        serial_println!("-- {:?}", frame);
        self.next += 1;
        frame
    }
}

pub fn init(memory_map_tag: &'static MemoryMapTag) -> Result<(), MapToError<Size2MiB>> {
    let mut frame_allocator = unsafe { AreaFrameAllocator::new(memory_map_tag) };
    // We haven't mapped physical memory to an offset yet. So, we use the kernel offset for now.
    let mut mapper = unsafe { mapper(VirtAddr::new(meta::kernel_offset() as u64)) };

    let kernel_size = meta::kernel_end() - meta::kernel_begin();

    // Todo: This is a temporary fix. This needs to be handled properly in the page fault handler.
    let phys_frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(kernel_size));
    let page = Page::<Size2MiB>::containing_address(VirtAddr::new(kernel_size + meta::kernel_offset()));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;
    unsafe {
        mapper.map_to(page, phys_frame, flags, &mut frame_allocator)?.flush();
    }

    // Todo: Needs improvement, doesn't take into account non-contiguous memory regions.
    let pages = kernel_size / Size4KiB::SIZE;
    frame_allocator.next = pages as usize;

    map_memory_at_pmo(memory_map_tag, &mut mapper, &mut frame_allocator)?;
    map_heap(&mut mapper, &mut frame_allocator)?;

    Ok(())
}

pub fn map_memory_at_pmo(memory_map_tag: &'static MemoryMapTag,
                         mapper: &mut impl Mapper<Size2MiB>,
                         frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size2MiB>> {
    let total_available_memory = memory_map_tag.available_memory_areas().map(|area| area.size()).sum::<u64>();
    let total_available_memory = x86_64::align_up(total_available_memory, Size2MiB::SIZE);

    let page_range = {
        let pmm_start = VirtAddr::new(PHYSICAL_MEMORY_OFFSET);
        let pmm_end = VirtAddr::new(pmm_start.as_u64() + total_available_memory - 1);
        let pmm_start_page = Page::<Size2MiB>::containing_address(pmm_start);
        let pmm_end_page = Page::<Size2MiB>::containing_address(pmm_end);
        Page::<Size2MiB>::range_inclusive(pmm_start_page, pmm_end_page)
    };

    let frame_range = (0..total_available_memory).step_by(Size2MiB::SIZE as usize)
                                                 .map(|addr| PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(addr)));
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;

    for (page, frame) in zip(page_range, frame_range) {
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    Ok(())
}

pub fn map_heap(mapper: &mut impl Mapper<Size2MiB>,
                frame_allocator: &mut impl FrameAllocator<Size4KiB>) -> Result<(), MapToError<Size2MiB>> {
    let page = Page::<Size2MiB>::containing_address(VirtAddr::new(HEAP_START));

    // Todo: Fix this, it's a very brittle approach and will cause errors.
    let frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(0xA00000)); // Mapping at 10 MB mark.

    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;

    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }

    Ok(())
}
