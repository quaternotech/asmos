OUTPUT_FORMAT("elf64-x86-64")
ENTRY(start)

KERNEL_OFFSET = 0xFFFFFFFF80000000;

SECTIONS {
    . = 1M;

    .prelude ALIGN(4K) : {
		KEEP(*(.prelude.multiboot))
		KEEP(*(.prelude .prelude.*))
    }

    . += KERNEL_OFFSET;

	.text ALIGN(4K) : AT(ADDR(.text) - KERNEL_OFFSET) {
		*(.text .text.*)
	}

	.rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_OFFSET) {
        *(.rodata .rodata.*)
    }

	.data ALIGN(4K) : AT(ADDR(.data) - KERNEL_OFFSET) {
        *(.data .data.*)
    }

    .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_OFFSET) {
        *(.bss .bss.*)
    }

    .got ALIGN(4K) : AT(ADDR(.got) - KERNEL_OFFSET) {
        *(.got .got.*)
    }

	/DISCARD/ : {
		*(.comment .comment.*)
        *(.eh_frame .eh_frame.*)
        *(.gcc_except_table .gcc_except_table.*)
        *(.note .note.*)
        *(.rel.eh_frame .rel.eh_frame.*)
	}
}
