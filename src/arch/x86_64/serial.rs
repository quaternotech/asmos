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

use core::fmt::Arguments;
use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions;

lazy_static! {
    /// Serial communication through 16550 UART interface.
    static ref UART_3F8: Mutex<SerialPort> = {
        // On x86_64 architecture, the UART serial device is accessed through port-mapped I/O.
        const SERIAL_IO_PORT: u16 = 0x3F8;

        let mut port = unsafe { SerialPort::new(SERIAL_IO_PORT) };
        port.init();

        Mutex::new(port)
    };
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    instructions::interrupts::without_interrupts(|| {
        UART_3F8
            .lock()
            .write_fmt(args)
            .expect("failed to print to serial output");
    });
}
