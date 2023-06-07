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

use super::configurations::CONFIG_CORE_MEMORY_STACK_SIZE;

// The `stack_default` macro generates a default stack with a specific size.
// It creates a `Stack` struct with an underlying array of zeros. The size of
// the array is determined by the `CONFIG_CORE_MEMORY_STACK_SIZE` constant.
macro_rules! stack_default {
    () => {
        Stack([0; CONFIG_CORE_MEMORY_STACK_SIZE])
    };
}

#[no_mangle]
static KERNEL_STACK_SIZE: usize = CONFIG_CORE_MEMORY_STACK_SIZE;

#[repr(C, align(4096))]
struct Stack<const SIZE: usize>([u8; SIZE]);

#[no_mangle]
static mut KERNEL_STACK: Stack<{ CONFIG_CORE_MEMORY_STACK_SIZE }> = stack_default!();
