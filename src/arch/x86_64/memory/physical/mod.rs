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

use core::ops::Range;

use multiboot2::MemoryMapTag;
use x86_64::PhysAddr;
use x86_64::structures::paging::{FrameAllocator, PageSize};
use x86_64::structures::paging::frame::PhysFrameRange;
use x86_64::structures::paging::PhysFrame;

pub use bitmap::BitmapAllocator;
pub use dummy::DummyAllocator;
use crate::arch::meta::kernel_end;

mod bitmap;
mod dummy;

pub static mut PMM: Option<BitmapAllocator> = None;

pub fn init(memory_map_tag: &'static MemoryMapTag) -> Result<(), ()> {
    unsafe {
        PMM.replace(BitmapAllocator::new(memory_map_tag));
    }

    let pmm = unsafe { PMM.as_mut().unwrap() };

    let limit = kernel_end() + 2093056;
    // Mark pages occupied by kernel and memory allocator as used.
    while let Some(frame) = pmm.allocate_frame() {
        // todo: This is very brittle. Must fix in future.
        if frame.start_address().as_u64() >= limit {
            break;
        }
    }

    Ok(())
}

pub fn total_memory(memory_map_tag: &'static MemoryMapTag) -> u64 {
    memory_map_tag.available_memory_areas().map(|area| area.size()).sum::<u64>()
}

pub fn total_memory_aligned<S: PageSize>(memory_map_tag: &'static MemoryMapTag) -> u64 {
    x86_64::align_up(total_memory(memory_map_tag), S::SIZE)
}

pub fn get_usable_areas<S: PageSize>(memory_map_tag: &'static MemoryMapTag) -> impl Iterator<Item=Range<u64>> {
    memory_map_tag.available_memory_areas()
                  .map(
                      |area| {
                          x86_64::align_up(area.start_address(), S::SIZE)
                              ..
                              x86_64::align_down(area.end_address(), S::SIZE)
                      }
                  ).filter(|aligned_area| aligned_area.end - aligned_area.start >= S::SIZE)
}

pub fn get_frame_range<S: PageSize>(begin: u64, size: u64) -> PhysFrameRange<S> {
    let begin = PhysAddr::new(begin);
    let end = begin + size;
    let first = PhysFrame::<S>::containing_address(begin);
    let last = PhysFrame::<S>::containing_address(end);
    PhysFrame::<S>::range(first, last)
}
