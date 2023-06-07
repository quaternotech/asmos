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
