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

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::exceptions::{
    BreakpointException,
    DivisionError,
    DoubleFault,
    GeneralProtectionFault,
    PageFault,
};

lazy_static! {
    /// Interrupt Descriptor Table (IDT)
    ///
    /// The IDT is a binary data structure unique to the IA-32 and x86-64 architectures that serves as the
    /// Protected Mode and Long Mode equivalent of the Real Mode Interrupt Vector Table (IVT), providing the CPU
    /// with the location of the Interrupt Service Routines (ISR) for each interrupt vector.
    ///
    /// NOTE: Before implementing the IDT, ensure that a functional GDT is available.
    ///
    /// OS Dev Wiki: https://wiki.osdev.org/Interrupt_Descriptor_Table
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // Set division error handler.
        idt.divide_error.set_handler_fn(DivisionError::handler);

        // Set breakpoint handler.
        idt.breakpoint.set_handler_fn(BreakpointException::handler);

        // Set double fault handler and a dedicated stack index for it.
        unsafe {
            idt.double_fault.set_handler_fn(DoubleFault::handler)
                            .set_stack_index(DoubleFault::IST_INDEX as u16);
        }

        // Set general protection fault handler.
        idt.general_protection_fault.set_handler_fn(GeneralProtectionFault::handler);

        // Set page fault handler.
        idt.page_fault.set_handler_fn(PageFault::handler);

        idt
    };
}

pub fn init() -> Result<(), ()> {
    IDT.load();

    Ok(())
}
