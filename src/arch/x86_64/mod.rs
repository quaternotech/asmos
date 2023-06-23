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

use x86_64::instructions;

pub mod allocator;
pub mod exceptions;
pub mod framebuffer;
pub mod gdt;
pub mod idt;
pub mod memory;
pub mod preliminary;

pub mod meta;

pub mod serial;
mod fonts;

pub(crate) fn init(boot_info_addr: usize) {
    meta::init(boot_info_addr).expect("kernel failed to retrieve metadata");

    let boot_info = meta::multiboot_info();

    gdt::init().expect("kernel failed to initialize GDT");
    idt::init().expect("kernel failed to initialize IDT");

    let memory_map_tag = boot_info.memory_map_tag()
                                  .expect("the bootloader failed to provide memory map tag");
    memory::init(memory_map_tag).expect("kernel failed to initialize memory");
    // allocator::init().expect("kernel failed to initialize allocator");

    // let vbe_buffer_tag = boot_info.vbe_info_tag();
    // serial_println!("{:?}", vbe_buffer_tag);
    // let framebuffer_tag = boot_info.framebuffer_tag()
    //                                .expect("the bootloader failed to provide VBE framebuffer");
    // let framebuffer_tag = framebuffer_tag.expect("unrecognized VBE framebuffer");
    // framebuffer::init(framebuffer_tag).expect("kernel failed to initialize framebuffer");
}

pub(crate) fn hlt_loop() -> ! {
    loop {
        instructions::hlt();
        core::hint::spin_loop();
    }
}

pub fn get() -> &'static str {
    "x86_64"
}
