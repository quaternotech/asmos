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

use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{ExceptionVector, InterruptStackFrame, PageFaultErrorCode};

use crate::serial_println;

/// Division Error (#DE, 0x00)
///
/// The Division Error occurs when dividing any number by 0 using the `DIV` or `IDIV` instruction, or when the division
/// result is too large to be represented in the destination. Since a faulting `DIV` or `IDIV` instruction is very easy
/// to insert anywhere in the code, many OS developers use this exception to test whether their exception handling
/// code works.
///
/// The saved instruction pointer points to the `DIV` or `IDIV` instruction which caused the exception.
///
/// OS Dev Wiki: https://wiki.osdev.org/Exceptions#Division_Error
pub struct DivisionError;

impl DivisionError {
    pub const CODE: u8 = ExceptionVector::Division as u8;
    pub const MNEMONIC: &'static str = "DE";

    pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame) {
        panic!(
            "(#{}, {:#04X}) @ {:#?}",
            Self::MNEMONIC,
            Self::CODE,
            stack_frame,
        );
    }
}

/// Breakpoint Exception (#BP, 0x03)
///
/// A Breakpoint Exception occurs when the processor encounters a debug breakpoint instruction in enabling the program's
/// execution to be paused for the inspection of its current state.
///
/// OS Dev Wiki: https://wiki.osdev.org/Exceptions#Breakpoint
pub struct BreakpointException;

impl BreakpointException {
    pub const CODE: u8 = ExceptionVector::Breakpoint as u8;
    pub const MNEMONIC: &'static str = "BP";

    pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame) {
        serial_println!(
            "(#{}, {:#04X}) @ {:#?}",
            Self::MNEMONIC,
            Self::CODE,
            stack_frame,
        );
    }
}

/// Double Fault (#DF, 0x08)
///
/// A Double Fault occurs when the processor encounters an error while handling a prior exception, indicating a critical
/// system error that needs appropriate handling.
///
/// OS Dev Wiki: https://wiki.osdev.org/Exceptions#Double_Fault
pub struct DoubleFault;

impl DoubleFault {
    pub const IST_INDEX: usize = 0x0;
    pub const CODE: u8 = ExceptionVector::Double as u8;
    pub const MNEMONIC: &'static str = "DF";

    pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame, err_code: u64) -> ! {
        panic!(
            "(#{}, {:#04X}) @ {:#?}, E={}",
            Self::MNEMONIC,
            Self::CODE,
            stack_frame,
            err_code,
        );
    }
}

/// General Protection Fault (#GP, 0x0D)
///
/// A General Protection Fault may occur for various reasons. The most common are:
///
/// - Segment error (privilege, type, limit, read/write rights).
/// - Executing a privileged instruction while CPL != 0.
/// - Writing a 1 in a reserved register field or writing invalid value combinations (e.g. CR0 with PE=0 and PG=1).
/// - Referencing or accessing a null-descriptor.
///
/// The saved instruction pointer points to the instruction which caused the exception.
///
/// The General Protection Fault sets an error code, which is the segment selector index when the exception is segment
/// related. Otherwise, 0.
///
/// OS Dev Wiki: https://wiki.osdev.org/Exceptions#General_Protection_Fault
pub struct GeneralProtectionFault;

impl GeneralProtectionFault {
    pub const CODE: u8 = ExceptionVector::GeneralProtection as u8;
    pub const MNEMONIC: &'static str = "GP";

    pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame, err_code: u64) {
        let addr = Cr2::read();

        panic!(
            "(#{}, {:#04X}) @ {:#?}, E={}, ADDR={:?}",
            Self::MNEMONIC,
            Self::CODE,
            stack_frame,
            err_code,
            addr,
        );
    }
}

/// Page Fault (#PF, 0x0E)
///
/// The Page Fault occurs when a program tries to access a memory page that is not currently mapped or has invalid
/// permissions. Proper handling of page faults is crucial for virtual memory management and involves tasks like
/// swapping pages in and out of physical memory or allocating new memory pages.
///
/// OS Dev Wiki: https://wiki.osdev.org/Exceptions#Page_Fault
pub struct PageFault;

impl PageFault {
    pub const CODE: u8 = ExceptionVector::Page as u8;
    pub const MNEMONIC: &'static str = "PF";

    pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame, err_code: PageFaultErrorCode) {
        let addr = Cr2::read();

        panic!(
            "(#{}, {:#04X}) @ {:#?}, E={:#?}, ADDR={:#?}",
            Self::MNEMONIC,
            Self::CODE,
            stack_frame,
            err_code,
            addr,
        );
    }
}
