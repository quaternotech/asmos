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

use core::arch::global_asm;

pub mod configurations;
mod multiboot;
mod paging;
mod stack;

// This assembly file contains essential instructions for configuring fundamental system
// settings and transitioning into the long mode of the processor. By incorporating this
// file, low-level operations and directives are directly integrated into the Rust code,
// allowing precise control over system initialization and utilization of advanced
// hardware features.
global_asm!(include_str!("trampoline.s"));
