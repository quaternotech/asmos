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

use multiboot2::{ElfSectionsTag, MemoryMapTag};
use x86_64::structures::paging::{PageSize, Size4KiB};

use crate::serial_println;

pub fn init(elf_sections_tag: ElfSectionsTag, _memory_map_tag: &'static MemoryMapTag)
            -> Result<(), ()> {
    let mut mem_occupied = 0;
    for section in elf_sections_tag.sections() {
        mem_occupied += section.size();
    }

    let mem_needed = x86_64::align_up(mem_occupied, Size4KiB::SIZE);
    serial_println!("{} - {}", mem_occupied, mem_needed);

    Ok(())
}
