// MIT License
//
// Copyright (c) 2023 Mansoor Ahmed Memon.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

use super::exceptions::{BreakpointException, DoubleFaultException};

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

        // Set breakpoint handler.
        idt.breakpoint.set_handler_fn(BreakpointException::handler);

        // Set double fault handler and a dedicated stack index for it.
        unsafe {
            idt.double_fault.set_handler_fn(DoubleFaultException::handler)
                            .set_stack_index(DoubleFaultException::IST_INDEX as u16);
        }

        idt
    };
}

pub fn init() -> Result<(), ()> {
    IDT.load();

    Ok(())
}
