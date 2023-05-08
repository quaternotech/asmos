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

use super::configurations::CONFIG_CORE_MEMORY_INITIAL_MAPPING_SIZE;

macro_rules! table_size { () => { 4096 } }

macro_rules! table_default {
    () => { PageTable([0; table_size!()]) }
}

#[no_mangle]
static INITIAL_MAPPING_SIZE: usize = CONFIG_CORE_MEMORY_INITIAL_MAPPING_SIZE;

#[repr(C, align(4096))]
struct PageTable([u8; table_size!()]);

#[no_mangle]
static mut PT4: PageTable = table_default!();
#[no_mangle]
static mut PT3: PageTable = table_default!();
#[no_mangle]
static mut PT2: PageTable = table_default!();
