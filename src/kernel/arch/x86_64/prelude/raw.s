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

.global start

.section .rodata

.align 16
GDT:
    .quad 0
    GDT_CODE_SEGMENT = . - GDT
    .quad 0x00AF9A000000FFFF
    GDT_DATA_SEGMENT = . - GDT
    .quad 0x00CF92000000FFFF
GDT_END:

GDT_POINTER:
    .word GDT_END - GDT - 1
    .quad GDT - KERNEL_OFFSET

.set KERNEL_OFFSET, 0xFFFFFFFF80000000
.set KERNEL_STACK_PA, KERNEL_STACK - KERNEL_OFFSET
.set KERNEL_STACK_SIZE_PA, KERNEL_STACK_SIZE - KERNEL_OFFSET

.set PT4_PA, PT4 - KERNEL_OFFSET
.set PT3_PA, PT3 - KERNEL_OFFSET
.set PT2_PA, PT2 - KERNEL_OFFSET

.set INITIAL_MAPPING_SIZE_PA, INITIAL_MAPPING_SIZE - KERNEL_OFFSET

.set GDT_POINTER_PA, GDT_POINTER - KERNEL_OFFSET

.set START_HIGHER_HALF_KERNEL_PA, start_higher_half_kernel - KERNEL_OFFSET

.section .prelude.text, "ax", @progbits
.code32

start:
    cli

    mov esi, eax
    mov edi, ebx

    mov eax, offset KERNEL_STACK_PA
    add eax, KERNEL_STACK_SIZE_PA
    mov esp, eax

    call check_multiboot_support
    call check_cpuid_support
    call check_long_mode_support

    call set_up_page_tables
    call enable_paging

    lgdt [GDT_POINTER_PA]

    lea eax, [GDT_DATA_SEGMENT]
    mov ds, eax
    mov es, eax
    mov fs, eax
    mov gs, eax
    mov ss, eax

    ljmp 0x8, offset START_HIGHER_HALF_KERNEL_PA

    hlt


check_multiboot_support:
    cmp esi, 0x36D76289
    jne .no_multiboot
    ret
.no_multiboot:
    mov eax, 0x30
    hlt


check_cpuid_support:
    pushfd
    pop eax
    // Copy to ECX as well for comparing later on.
    mov ecx, eax
    // Flip the ID bit.
    xor eax, 0x200000
    // Copy EAX to FLAGS via the stack.
    push eax
    popfd
    // Copy FLAGS back to EAX (with the flipped bit if CPUID is supported).
    pushfd
    pop eax
    // Restore FLAGS from the old version stored in ECX (i.e. flipping the ID bit back if it was ever flipped).
    push ecx
    popfd

    // Compare EAX and ECX. If they are equal then that means the bit wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov eax, 0x31
    hlt

check_long_mode_support:
    // Test if extended processor info is available.
    mov eax, 0x80000000                                         // Implicit argument for cpuid.
    cpuid                                                       // Get highest supported argument.
    cmp eax, 0x80000001                                         // It needs to be at least 0x80000001.
    jb .no_long_mode                                            // If it's less, the CPU is too old for long mode.

    // Use extended info to test if long mode is available.
    mov eax, 0x80000001                                         // Argument for extended processor info.
    cpuid                                                       // Returns various feature bits in ecx and edx.
    test edx, 0x20000000                                        // Test if the LM-bit is set in the D-register.
    je .no_long_mode                                            // If it's not set, there is no long mode.
    ret
.no_long_mode:
    mov al, 0x32
    hlt


set_up_page_tables:
    mov eax, offset PT4_PA
    mov ebx, offset PT3_PA + 0b11
    mov dword ptr [eax], ebx

    mov eax, offset PT3_PA
    mov ebx, offset PT2_PA + 0b11
    mov dword ptr [eax], ebx

    mov eax, offset PT2_PA
    mov ebx, 0b10000011

    mov ecx, 0
    mov edx, INITIAL_MAPPING_SIZE_PA
    shr edx, 21
.map_page_directory:
    mov dword ptr [eax + ecx * 8], ebx

    add ebx, 0x200000

    inc ecx
    cmp ecx, edx
    jne .map_page_directory

    ret


enable_paging:
    // Enable flags in CR4 register:
    //  1. Protected-mode Virtual Interrupts (PVI)          [1]
    //  2. Physical Address Extension (PAE)                 [5]
    //  3. Page Global Enabled (PGE)                        [7]
    mov eax, cr4
    or eax, (1 << 7) | (1 << 5) | (1 << 1)
    mov cr4, eax

    // Load PML4 to CR3 register (CPU uses this to access the PML4 table).
    mov eax, offset PT4_PA
    mov cr3, eax

    // Set the long mode bit in the Extended Feature Enable Register (EFER).
    mov ecx, 0xC0000080
    // Enable flags in EFER MSR:
    //  1. Long Mode Enable (LME)           [8]
    //  2. No-Execute Enable (NXE)          [11]
    rdmsr
    or eax, (1 << 11) | (1 << 8)
    wrmsr

    // Enable flags in CR0 register:
    //  1. Write Protect (WP)               [16]
    //  2. Paging (PG)                      [31]
    mov eax, cr0
    or eax, (1 << 31) | (1 << 16)
    mov cr0, eax

    ret
