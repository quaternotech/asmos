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

use x86_64::structures::idt::InterruptStackFrame;

use crate::serial_println;

/// Breakpoint Exception (#BP, 0x03)
///
/// A breakpoint exception occurs when the processor encounters a debug breakpoint instruction in enabling the
/// program's execution to be paused for the inspection of its current state.
///
/// OS Dev Wiki: https://wiki.osdev.org/Exceptions#Breakpoint
pub struct BreakpointException;

impl BreakpointException {
    pub const CODE: u8 = 0x03;
    pub const MNEMONIC: &'static str = "#BP";

    pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame) {
        serial_println!(
            "({}, {:#04X}) @ {:#?}",
            Self::MNEMONIC,
            Self::CODE,
            stack_frame
        );
    }
}

/// Double Fault Exception (#DF, 0x08)
///
/// A double fault exception occurs when the processor encounters an error while handling a prior exception,
/// indicating a critical system error that needs appropriate handling.
///
/// OS Dev Wiki: https://wiki.osdev.org/Exceptions#Double_Fault
pub struct DoubleFaultException;

impl DoubleFaultException {
    pub const IST_INDEX: usize = 0x0;
    pub const CODE: u8 = 0x08;
    pub const MNEMONIC: &'static str = "#DF";

    pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame, err_code: u64) -> ! {
        panic!(
            "({}, {:#04X}) @ {:#?}, E={}",
            Self::MNEMONIC,
            Self::CODE,
            stack_frame,
            err_code
        );
    }
}
