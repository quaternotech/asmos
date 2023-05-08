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

use super::configurations::CONFIG_CORE_MEMORY_STACK_SIZE;

macro_rules! stack_default {
    () => { Stack([0; CONFIG_CORE_MEMORY_STACK_SIZE]) };
}

#[no_mangle]
static KERNEL_STACK_SIZE: usize = CONFIG_CORE_MEMORY_STACK_SIZE;

#[repr(C, align(4096))]
struct Stack<const SIZE: usize>([u8; SIZE]);

#[no_mangle]
static mut KERNEL_STACK: Stack<{ CONFIG_CORE_MEMORY_STACK_SIZE }> = stack_default!();
