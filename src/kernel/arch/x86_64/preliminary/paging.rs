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

// The page table consists of 512 entries, with each entry having a size of 8 bytes.
// Therefore, the total size of the page table is calculated as 512 * 8 = 4096 bytes.
macro_rules! table_size {
    () => {
        4096
    };
}

// The `table_default` macro generates a default page table with a specific size.
// It creates a `PageTable` struct with an underlying array of zeros. The size of
// the array is determined by the `table_size!()` macro.
macro_rules! table_default {
    () => {
        PageTable([0; table_size!()])
    };
}

// The initial mapping size used for the core memory. This value is retrieved
// from the CONFIG_CORE_MEMORY_INITIAL_MAPPING_SIZE configuration option
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
