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

use core::iter;

use lazy_static::lazy_static;
use multiboot2::MemoryMapTag;
use spin::Mutex;
use x86_64::instructions::interrupts;
use x86_64::PhysAddr;
use x86_64::structures::paging::{PageTableFlags, PhysFrame, Size4KiB};
use x86_64::structures::paging::{FrameAllocator, Mapper, PageSize};
use x86_64::structures::paging::frame::PhysFrameRange;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::page::PageRange;

use super::{get_frame_range, get_page_range};
use super::meta;

lazy_static! {
    pub static ref PHYSICAL_MEMORY_MANAGER: Mutex<Option<PMM>> = Mutex::new(None);
}

pub struct PMM {
    memory_map_tag: &'static MemoryMapTag,
    next: usize,
}

impl PMM {
    unsafe fn new(memory_map_tag: &'static MemoryMapTag) -> Self {
        PMM { memory_map_tag, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item=PhysFrame> {
        self.memory_map_tag.available_memory_areas()
            .map(
                |area| {
                    x86_64::align_up(area.start_address(), Size4KiB::SIZE)
                        ..
                        x86_64::align_down(area.end_address(), Size4KiB::SIZE)
                }
            ).filter(|aligned_area| aligned_area.end - aligned_area.start >= Size4KiB::SIZE)
            .flat_map(|range| range.step_by(Size4KiB::SIZE as usize))
            .map(|address| PhysFrame::containing_address(PhysAddr::new(address)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for PMM {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

pub unsafe fn init(memory_map_tag: &'static MemoryMapTag) -> Result<(), ()> {
    interrupts::without_interrupts(|| {
        PHYSICAL_MEMORY_MANAGER.lock()
                               .replace(PMM::new(memory_map_tag));
    });

    let mut pmm = PHYSICAL_MEMORY_MANAGER.lock();
    let pmm = pmm.as_mut().unwrap();

    // Exclude frames that are reserved for the kernel.
    let kernel_end = meta::kernel_end();
    for frame in pmm.usable_frames() {
        if frame.start_address().as_u64() >= kernel_end {
            break;
        }
        pmm.next += 1;
    }

    Ok(())
}

pub unsafe fn identity_map_range(mapper: &mut impl Mapper<Size4KiB>,
                                 flags: PageTableFlags,
                                 frame_range: PhysFrameRange<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let mut frame_allocator = PHYSICAL_MEMORY_MANAGER.lock();
    let frame_allocator = frame_allocator.as_mut().unwrap();

    for frame in frame_range {
        mapper.identity_map(frame, flags, frame_allocator)?.flush();
    }

    Ok(())
}

pub unsafe fn map_range<S: PageSize>(mapper: &mut impl Mapper<S>,
                                     flags: PageTableFlags,
                                     physical_offset: u64,
                                     virtual_offset: u64,
                                     size: u64) -> Result<(), MapToError<S>> {
    let mut frame_allocator = PHYSICAL_MEMORY_MANAGER.lock();
    let frame_allocator = frame_allocator.as_mut().unwrap();

    let frame_range = get_frame_range::<S>(physical_offset, size);
    let page_range = get_page_range::<S>(virtual_offset, size);

    for (frame, page) in iter::zip(frame_range, page_range) {
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    Ok(())
}

pub unsafe fn allocate_range(mapper: &mut impl Mapper<Size4KiB>,
                             flags: PageTableFlags,
                             page_range: PageRange) -> Result<(), MapToError<Size4KiB>> {
    let mut frame_allocator = PHYSICAL_MEMORY_MANAGER.lock();
    let frame_allocator = frame_allocator.as_mut().unwrap();

    for page in page_range {
        let frame = frame_allocator.allocate_frame()
                                   .expect("physical memory manager ran out of memory");
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }

    Ok(())
}
