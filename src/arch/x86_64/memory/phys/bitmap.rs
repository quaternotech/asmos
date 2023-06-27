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

use super::{get_frame_range, get_usable_areas};
use super::DummyAllocator;

#[derive(Debug)]
struct Bitmap {
    num_frames: usize,
    start: *mut u8,
}

impl Bitmap {
    fn new(num_frames: usize, start: *mut u8) -> Self {
        Self { num_frames, start }
    }

    fn bytes(&self) -> usize { (self.num_frames + 7) >> 3 }

    fn first_free_frame(&self) -> Option<usize> {
        let size = self.bytes();
        let ptr = self.start;

        let mut i = 0;
        let mut byte = 0;
        while i < size {
            byte = unsafe { *ptr.offset(i as isize) };
            if byte != u8::MAX {
                break;
            }
            i += 1;
        }
        if i == size {
            return None;
        }

        let mut b = 0;
        while b < u8::BITS {
            if byte & (1 << b) == 0 {
                break;
            }
            b += 1;
        }

        let frame_index = (i * u8::BITS as usize) + b as usize;
        if frame_index >= self.num_frames { None } else { Some(frame_index) }
    }

    fn next(&mut self) -> Option<usize> {
        if let Some(frame_index) = self.first_free_frame() {
            self.set_one(frame_index);
            Some(frame_index)
        } else {
            None
        }
    }

    fn is_zero(&self, frame_index: usize) -> bool { self.get_bit(frame_index) == 0 }

    fn is_one(&self, frame_index: usize) -> bool { !self.is_zero(frame_index) }

    fn coords(&self, frame_index: usize) -> (usize, u8) {
        assert!(frame_index < self.num_frames);

        let byte_index = frame_index / u8::BITS as usize;
        let bit_index = (frame_index % u8::BITS as usize) as u8;
        (byte_index, bit_index)
    }

    fn get_bit(&self, frame_index: usize) -> u8 {
        let (byte_index, bit_index) = self.coords(frame_index);
        let byte = unsafe { *self.start.offset(byte_index as isize) };
        (byte >> bit_index) & 1
    }

    fn set_bit(&self, frame_index: usize, value: bool) {
        let (byte_index, bit_index) = self.coords(frame_index);
        unsafe {
            let ptr = self.start.offset(byte_index as isize);
            *ptr = (*ptr & !(1 << bit_index)) | (value as u8) << bit_index;
        };
    }

    fn set_one(&mut self, frame_index: usize) {
        let (byte_index, bit_index) = self.coords(frame_index);
        let ptr = unsafe { self.start.offset(byte_index as isize) };
        unsafe { *ptr |= 1 << bit_index; }
    }

    fn set_zero(&mut self, frame_index: usize) {
        let (byte_index, bit_index) = self.coords(frame_index);
        let ptr = unsafe { self.start.offset(byte_index as isize) };
        unsafe { *ptr &= !(1 << bit_index); }
    }

    fn toggle_bit(&mut self, frame_index: usize) {
        let (byte_index, bit_index) = self.coords(frame_index);
        let ptr = unsafe { self.start.offset(byte_index as isize) };
        unsafe { *ptr ^= 1 << bit_index };
    }

    fn extra_bits(&self) -> u8 {
        ((self.bytes() * u8::BITS as usize) - self.num_frames) as u8
    }
}

#[derive(Debug)]
#[repr(C)]
struct MemoryChunk {
    num_frames: usize,
    address: usize,
    bitmap: *mut u8,
    next: *mut MemoryChunk,
}

impl MemoryChunk {
    unsafe fn reference(ptr: *mut Self) -> &'static mut Self {
        &mut *ptr
    }

    fn set(&mut self, size: usize, address: usize, bitmap: *mut u8) {
        self.num_frames = size;
        self.address = address;
        self.bitmap = bitmap;
        self.next = ptr::null_mut();
    }

    fn bitmap(&self) -> Bitmap {
        Bitmap::new(self.num_frames, self.bitmap)
    }

    fn size(&self) -> usize {
        self.num_frames * Size4KiB::SIZE as usize
    }
}

#[derive(Debug)]
pub struct BitmapAllocator {
    head: *mut MemoryChunk,
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

        // Dynamically allocate a memory for phys memory manager's data structures.
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
            let size = frame.size() / Size4KiB::SIZE as usize;
            let start_address = frame.start as usize;
            let bitmap = (cur as usize + mem::size_of::<MemoryChunk>()) as *mut u8;
            unsafe { (*cur).set(size, start_address, bitmap) };
            let bitmap_size = unsafe { (*cur).bitmap().bytes() };
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
        let mut cur = self.head;
        while cur != ptr::null_mut() {
            let node = unsafe { MemoryChunk::reference(cur) };
            if let Some(frame_index) = node.bitmap().next() {
                let address = node.address + (frame_index * 4096);
                frame = Some(PhysFrame::containing_address(PhysAddr::new(address as u64)));
                break;
            }
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
