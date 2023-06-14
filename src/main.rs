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

#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]

extern crate alloc;

use alloc::vec;
use core::panic::PanicInfo;

use asmos::serial_println;

#[no_mangle]
pub extern "C" fn k_main(boot_info_addr: usize) -> ! {
    asmos::init(boot_info_addr);

    asmos::hlt_loop();
}

#[panic_handler]
fn on_panic(panic_info: &PanicInfo) -> ! {
    serial_println!("EXCEPTION OCCURRED => {:#?}", panic_info.message());

    asmos::hlt_loop();
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
