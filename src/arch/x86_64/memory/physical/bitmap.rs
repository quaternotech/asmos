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

use core::{iter, mem, ptr};
use core::iter::TrustedRandomAccessNoCoerce;

use multiboot2::MemoryMapTag;
use x86_64::PhysAddr;
use x86_64::structures::paging::{PageTableFlags, PhysFrame, Size2MiB, Size4KiB};
use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, Mapper, PageSize};

use crate::arch::memory::{get_mapper, get_page_range};
use crate::arch::meta;
use crate::serial_println;

use super::{get_frame_range, get_usable_areas};
use super::DummyAllocator;

#[derive(Debug)]
#[repr(C)]
pub struct MemoryChunk {
    pub size: usize,
    pub start_address: usize,
    pub bitmap: *mut u8,
    pub next: *mut MemoryChunk,
}

impl MemoryChunk {
    pub unsafe fn reference(ptr: *mut Self) -> &'static mut Self {
        &mut *ptr
    }

    pub unsafe fn initialize(&mut self, size: usize, start_address: usize, bitmap: *mut u8) {
        self.size = size;
        self.start_address = start_address;
        self.bitmap = bitmap;
        self.next = ptr::null_mut();
    }

    pub fn bitmap_size(&self) -> usize {
        (x86_64::align_up((self.size / Size4KiB::SIZE as usize) as u64, u8::BITS.into()) / u8::BITS as u64) as usize
    }
}

#[derive(Debug)]
pub struct BitmapAllocator {
    pub head: *mut MemoryChunk,
}

impl BitmapAllocator {
    pub fn new(memory_map_tag: &'static MemoryMapTag) -> Self {
        let p_kernel_end = meta::kernel_end();
        let v_kernel_end = p_kernel_end + meta::kernel_offset();
        let struct_size = mem::size_of::<MemoryChunk>();

        let frames = || get_usable_areas::<Size4KiB>(memory_map_tag);

        let n = frames().count();
        assert!(n > 0);
        let n_struct_size = (n * struct_size) as u64;
        let n_bitmap_size = frames().map(|range| {
            x86_64::align_up((range.size() / Size4KiB::SIZE as usize) as u64, u8::BITS.into()) / u8::BITS as u64
        }).sum::<u64>();
        let total_memory = n_struct_size + n_bitmap_size;
        let memory_required = x86_64::align_up(total_memory, Size2MiB::SIZE);

        // Dynamically allocate a memory for physical memory manager's data structures.
        let mut mapper = unsafe { get_mapper(meta::kernel_offset()) };
        let mut allocator = DummyAllocator {};
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::HUGE_PAGE;
        let page_range = get_page_range::<Size2MiB>(v_kernel_end, memory_required);
        let frame_range = get_frame_range::<Size2MiB>(p_kernel_end, memory_required);
        for (page, frame) in iter::zip(page_range, frame_range) {
            unsafe { mapper.map_to(page, frame, flags, &mut allocator).unwrap().flush() };
        }

        let head = v_kernel_end as *mut MemoryChunk;

        let mut cur = head;
        for (i, frame) in frames().enumerate() {
            let size = frame.size();
            let start_address = frame.start as usize;
            let bitmap = (cur as usize + mem::size_of::<MemoryChunk>()) as *mut u8;
            unsafe { (*cur).initialize(size, start_address, bitmap) };
            let bitmap_size = unsafe { (*cur).bitmap_size() };
            if i < n - 1 {
                unsafe {
                    (*cur).next = (bitmap as usize + bitmap_size as usize) as *mut MemoryChunk;
                    cur = (*cur).next;
                }
            }
        }

        Self { head }
    }
}

unsafe impl FrameAllocator<Size4KiB> for BitmapAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let mut frame = None;
        let mut frame_found = false;
        let mut cur = self.head;
        while cur != ptr::null_mut() && !frame_found {
            let node = unsafe { MemoryChunk::reference(cur) };
            let bitmap = node.bitmap;
            let bitmap_size = node.bitmap_size();
            let mut i = 0;
            while i < bitmap_size {
                let value = unsafe { *((bitmap as usize + i) as *mut u8) };
                if value != u8::MAX {
                    frame_found = true;
                    break;
                }

                // todo: handle extra bits. Ignoring it for now.

                i += 1;
            }

            let ptr = (bitmap as usize + i) as *mut u8;
            let value = unsafe { *ptr };
            let mut j = 0;
            while j < 8 {
                if ((value >> j) & 1) == 0 {
                    break;
                }
                j += 1;
            }
            unsafe {
                // todo: fix "attempt to shift left with overflow"
                // occurs in debug mode.
                assert!(j < 8); // fails
                *ptr = value | (1 << j);
            }

            let address = node.start_address + (((i * 8) + j) * 4096);
            frame = Some(PhysFrame::containing_address(PhysAddr::new(address as u64)));

            cur = unsafe { (*cur).next };
        }

        frame
    }
}

impl FrameDeallocator<Size4KiB> for BitmapAllocator {
    unsafe fn deallocate_frame(&mut self, _frame: PhysFrame<Size4KiB>) {
        // todo: implement.
    }
}
