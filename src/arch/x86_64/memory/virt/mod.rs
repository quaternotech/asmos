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

use x86_64::structures::paging::{Mapper, PageSize, PageTableFlags, Size4KiB};
use x86_64::structures::paging::frame::PhysFrameRange;
use x86_64::structures::paging::mapper::{MapToError, UnmapError};
use x86_64::structures::paging::page::PageRange;

use crate::arch::memory::get_page_range;
use crate::arch::memory::physical::PMM;
use crate::arch::memory::physical::get_frame_range;

pub unsafe fn identity_map_range(mapper: &mut impl Mapper<Size4KiB>,
                                 flags: PageTableFlags,
                                 frame_range: PhysFrameRange<Size4KiB>) -> Result<(), MapToError<Size4KiB>> {
    let frame_allocator = PMM.as_mut().unwrap();

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
    let frame_allocator = PMM.as_mut().unwrap();

    let frame_range = get_frame_range::<S>(physical_offset, size);
    let page_range = get_page_range::<S>(virtual_offset, size);

    for (frame, page) in iter::zip(frame_range, page_range) {
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        }
    }

    Ok(())
}

pub unsafe fn unmap_range<S: PageSize>(mapper: &mut impl Mapper<S>,
                                       page_range: PageRange<S>) -> Result<(), UnmapError> {
    for page in page_range {
        mapper.unmap(page)?.1.flush();
    }

    Ok(())
}
