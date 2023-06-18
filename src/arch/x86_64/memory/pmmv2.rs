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

use core::{iter, ptr};
use core::mem::size_of;

use lazy_static::lazy_static;
use multiboot2::MemoryMapTag;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, Mapper, PageSize, PageTableFlags};
use x86_64::structures::paging::{PhysFrame, Size2MiB, Size4KiB};

use crate::arch::meta;
use crate::arch::meta::kernel_end;
use crate::arch::x86_64::memory::{get_frame_range, get_mapper, get_page_range};
use crate::serial_println;

use super::physical::mm::dummy::DummyPMM;

static mut INNER_PMM: Option<InnerPMM> = None;

#[repr(C)]
struct MemoryArea {
    start_address: u64,
    size: u64,
    next: *mut MemoryArea,
}


// Pmm's job is to find pages of required size in memory.
// Creating a bitmap allocator with a bitmap for each region.
// So, my bitmap allocator will maintain a linked list of all regions.
// Then, each region will have a bitmap of some size (still not sure about alignment of it)
// From those bitmaps, they might have a few extra bits at the end, but these bits can be safely
// cross-checked using region size, if region size is full, it means those bits are extra.
// So, now, we need to figure out, how to implement this.
struct InnerPMM {
    first: *mut MemoryArea,
}

macro_rules! make_buffer {
    ($name:ident) => { static mut $name: [u8; 4096] = [0; 4096]; }
}

impl InnerPMM {
    fn new(memory_map_tag: &'static MemoryMapTag) -> Self {
        let memory_areas = memory_map_tag.available_memory_areas()
                                         .map(
                                             |area| {
                                                 x86_64::align_up(area.start_address(), Size4KiB::SIZE)
                                                     ..
                                                     x86_64::align_down(area.end_address(), Size4KiB::SIZE)
                                             }
                                         ).filter(|aligned_area| aligned_area.end - aligned_area.start >= Size4KiB::SIZE);

        let mut count = 0;
        let mut total_bitmap_size = 0;
        for area in memory_areas {
            let size = area.end - area.start;
            let pages_required = size / Size4KiB::SIZE;
            let bitmap_size = (pages_required + 7) / 8;
            total_bitmap_size += bitmap_size;
            count += 1;
            serial_println!("size={} pages={} bitmap={}", size, pages_required, bitmap_size);
        }
        serial_println!("{:?}", size_of::<MemoryArea>());
        serial_println!("{}", total_bitmap_size);

        let structures_size = count * size_of::<MemoryArea>();
        let memory_required = total_bitmap_size + structures_size as u64;
        let memory_required = x86_64::align_up(memory_required, Size2MiB::SIZE);
        serial_println!("{:?}", memory_required);

        let mut mapper = unsafe { get_mapper(meta::kernel_offset()) };
        let mut allocator = DummyPMM {};

        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;

        let page_range = get_page_range::<Size2MiB>(meta::kernel_end() + meta::kernel_offset(), memory_required);
        let frame_range = get_frame_range::<Size2MiB>(meta::kernel_end(), memory_required);

        // here we dynamically allocated a region for pmm data structures.
        for (page, frame) in iter::zip(page_range, frame_range) {
            unsafe { mapper.map_to(page, frame, flags, &mut allocator).unwrap().flush() };
        }

        let memory_areas = memory_map_tag.available_memory_areas()
                                         .map(
                                             |area| {
                                                 x86_64::align_up(area.start_address(), Size4KiB::SIZE)
                                                     ..
                                                     x86_64::align_down(area.end_address(), Size4KiB::SIZE)
                                             }
                                         ).filter(|aligned_area| aligned_area.end - aligned_area.start >= Size4KiB::SIZE);

        let abs_first = (kernel_end() + meta::kernel_offset()) as *mut MemoryArea;
        let mut prev = ptr::null_mut();
        let mut first = abs_first;

        let mut i = 0;
        for area in memory_areas {
            let size = area.end - area.start;
            let pages_required = size / Size4KiB::SIZE;
            let bitmap_size = (pages_required + 7) / 8;

            unsafe {
                (*first).start_address = area.start as u64;
                (*first).size = size;
                (*first).next = (first as usize + size_of::<MemoryArea>() + bitmap_size as usize) as *mut MemoryArea;
                prev = first;
                first = (*first).next;
                i += 1;
            }
        }

        serial_println!("{:?}", i);
        unsafe {
            (*prev).next = ptr::null_mut();
        }
        // let offset = meta::kernel_end();
        // let first_page = Page::<Size2MiB>::containing_address(VirtAddr::new(offset + meta::kernel_offset()));
        // let first_frame = PhysFrame::<Size2MiB>::containing_address(PhysAddr::new(offset));
        // serial_println!("{:?}", (first_page, first_frame));
        //
        // for p in 0..pages_required {
        // }

        Self { first: abs_first }
    }
}

unsafe impl FrameAllocator<Size4KiB> for InnerPMM {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}

unsafe impl FrameAllocator<Size2MiB> for InnerPMM {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size2MiB>> {
        None
    }
}

pub struct PMM {
    __private: u8,
}

impl Default for PMM {
    fn default() -> Self {
        PMM { __private: 0 }
    }
}

unsafe impl FrameAllocator<Size4KiB> for PMM {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let mut inner_pmm = unsafe { INNER_PMM.as_mut() }.unwrap();

        inner_pmm.allocate_frame()
    }
}

unsafe impl FrameAllocator<Size2MiB> for PMM {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size2MiB>> {
        let mut inner_pmm = unsafe { INNER_PMM.as_mut() }.unwrap();

        inner_pmm.allocate_frame()
    }
}

pub fn init(memory_map_tag: &'static MemoryMapTag) -> Result<(), ()> {
    unsafe {
        INNER_PMM.replace(InnerPMM::new(memory_map_tag));

        let inner_pmm = INNER_PMM.as_mut().unwrap();
        serial_println!("first={:?}", inner_pmm.first);

        let mut i = 1;
        let mut cur = inner_pmm.first;
        while cur != ptr::null_mut() {
            let area = &*cur;
            serial_println!("{} cur={:?} start={:X} size={:?} next={:?}", i, cur, area.start_address, area.size, area.next);
            cur = (*cur).next;
            i += 1;
        }
    }

    Ok(())
}

pub fn get_instance() -> Result<PMM, ()> {
    unsafe {
        if INNER_PMM.is_none() {
            Err(())
        } else {
            Ok(PMM::default())
        }
    }
}
