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

use multiboot2::FramebufferTag;
use x86_64::PhysAddr;

use crate::arch::x86_64::memory;
use crate::serial_println;

pub fn init(framebuffer_tag: FramebufferTag) -> Result<(), ()> {
    serial_println!("{:X}", framebuffer_tag.address);
    serial_println!("{:?}", framebuffer_tag.bpp);
    serial_println!("{:?}", framebuffer_tag.pitch);
    serial_println!("{:?} {:?}", framebuffer_tag.width, framebuffer_tag.height);
    serial_println!("{:?}", framebuffer_tag.buffer_type);

    let vram = memory::p2v(PhysAddr::new(framebuffer_tag.address)).as_u64();
    let pix_width = framebuffer_tag.bpp / 8;

    for i in 0..10 {
        let ch = &super::fonts::CHARACTERS[i];
        for y in 0..8 {
            let r = ch[y];
            for x in 0..8 {
                let b = r >> x & 1;
                put_pixel(vram, framebuffer_tag.pitch, pix_width, ((i * 8) + x) as u32, y as u32, (b * 0xFFFFFF) as u32);
            }
        }
    }

    Ok(())
}

pub fn put_pixel(vram: u64, pitch: u32, pix_width: u8, x: u32, y: u32, color: u32) {
    let dest = (vram + (y * pitch) as u64 + (x * pix_width as u32) as u64) as *mut u32;
    unsafe { *dest = color };
}
