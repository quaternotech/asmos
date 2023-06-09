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

use multiboot2::BootInformation;

macro_rules! foreign_symbol {
    ($symbol:ident) => {
        unsafe { &$symbol as *const u8 as usize }
    };
}

extern "C" {
    static KERNEL_OFFSET: u8;
}

static mut MULTIBOOT_INFO: Option<BootInformation> = None;

pub fn init(boot_info_addr: usize) -> Result<(), ()> {
    unsafe {
        MULTIBOOT_INFO = multiboot2::load(boot_info_addr).ok();
    }

    Ok(())
}

#[allow(dead_code)]
pub fn multiboot_info() -> &'static BootInformation {
    unsafe { MULTIBOOT_INFO.as_ref().unwrap() }
}

#[allow(dead_code)]
pub fn kernel_offset() -> usize {
    foreign_symbol!(KERNEL_OFFSET)
}
