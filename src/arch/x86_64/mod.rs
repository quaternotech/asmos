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

mod exceptions;
mod gdt;
mod idt;
mod memory;
mod meta;
mod preliminary;

pub mod serial;

pub fn init(boot_info_addr: usize) {
    meta::init(boot_info_addr).expect("kernel failed to retrieve metadata");

    let boot_info = meta::multiboot_info();

    gdt::init().expect("kernel failed to initialize GDT");
    idt::init().expect("kernel failed to initialize IDT");

    let elf_sections_tag = boot_info.elf_sections_tag()
                                    .expect("the bootloader failed to provide elf sections tag");
    let memory_map_tag = boot_info.memory_map_tag()
                                  .expect("the bootloader failed to provide memory map tag");
    memory::init(elf_sections_tag, memory_map_tag).expect("kernel failed to initialize memory");
}

pub fn hlt_loop() -> ! {
    loop {
        instructions::hlt();
        core::hint::spin_loop();
    }
}