/*
 * Copyright 2023 Quaterno LLC
 *
 * Author: Mansoor Ahmed Memon <mansoorahmed.one@gmail.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

OUTPUT_FORMAT("elf64-x86-64")
ENTRY(start)

KERNEL_OFFSET = 0xFFFFFFFF80000000;

SECTIONS {
    . = 1M;
    KERNEL_BEGIN = .;

    .preliminary ALIGN(4K) : {
		KEEP(*(.preliminary.multiboot))
		KEEP(*(.preliminary*))
    }

    . += KERNEL_OFFSET;

	.text ALIGN(4K) : AT(ADDR(.text) - KERNEL_OFFSET) {
		*(.text*)
	}

	.rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        *(.rodata*)
    }

	.data ALIGN(4K) : AT(ADDR(.data) - KERNEL_OFFSET) {
        *(.data*)
    }

    .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_OFFSET) {
        *(.bss*)
    }

    .got ALIGN(4K) : AT(ADDR(.got) - KERNEL_OFFSET) {
        *(.got*)
    }

    .debug ALIGN(4K) - KERNEL_OFFSET : {
        *(.debug_abbrev)
        *(.debug_info)
        *(.debug_line)
        *(.debug_str)

        KEEP(*(.debug*))
    }

    .symtab . - KERNEL_OFFSET : {
        KEEP(*(.symtab))
    }

    .strtab . - KERNEL_OFFSET : {
        KEEP(*(.strtab))
    }

    .shstrtab . - KERNEL_OFFSET : {
        KEEP(*(.shstrtab))
    }

    .stab . - KERNEL_OFFSET : {
        KEEP(*(.stab))
    }

    .stabstr . - KERNEL_OFFSET : {
        KEEP(*(.stabstr))
    }

    . = ALIGN(4K);
    KERNEL_END = .;

	/DISCARD/ : {
        *(.comment*)
        *(.eh_frame*)
        *(.gcc_except_table*)
        *(.note*)
        *(.rel.eh_frame*)
	}
}
