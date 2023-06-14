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

use crate::define;

define!(header_magic, 0xE85250D6);
define!(header_arch, 0);
define!(header_size, core::mem::size_of::<MultibootHeader>() as u32);
define!(header_checksum, -(header_magic!() + header_arch!() + header_size!() as isize) as u32);

define!(header_tag_end, 0);
define!(header_tag_info_request, 1);
// define!(header_tag_address, 2);
// define!(header_tag_entry_address, 3);
define!(header_tag_console_flags, 4);
define!(header_tag_framebuffer, 5);
// define!(header_tag_module_align, 6);
// define!(header_tag_efi_bs, 7);
// define!(header_tag_entry_address_efi32, 8);
// define!(header_tag_entry_address_efi64, 9);
// define!(header_tag_relocatable, 10);

// define!(tag_type_end, 0);
// define!(tag_type_cmdline, 1);
// define!(tag_type_boot_loader_name, 2);
// define!(tag_type_module, 3);
// define!(tag_type_basic_mem_info, 4);
// define!(tag_type_boot_dev, 5);
define!(tag_type_mem_map, 6);
// define!(tag_type_vbe, 7);
// define!(tag_type_framebuffer, 8);
// define!(tag_type_elf_sections, 9);
// define!(tag_type_apm, 10);
// define!(tag_type_efi32, 11);
// define!(tag_type_efi64, 12);
// define!(tag_type_sys_man_bios, 13);
// define!(tag_type_acpi_old, 14);
// define!(tag_type_acpi_new, 15);
// define!(tag_type_network, 16);
// define!(tag_type_efi_mem_map, 17);
// define!(tag_type_efi_bs, 18);
// define!(tag_type_efi32_ih, 19);
// define!(tag_type_efi64_ih, 20);
// define!(tag_type_load_base_addr, 21);

define!(flags_none, 0);
define!(flags_framebuffer, 1);

macro_rules! tag {
    ($type_:expr, $flags:expr, $size:expr) => {
        MultibootHeaderTag {
            type_: $type_,
            flags: $flags,
            size: $size,
        }
    };
}

macro_rules! tag_info_request {
    () => {
        tag!(
            header_tag_info_request!(),
            flags_none!(),
            core::mem::size_of::<MultibootInfoRequest>() as u32
        )
    };
}

macro_rules! tag_console_request {
    () => {
        tag!(
            header_tag_console_flags!(),
            flags_none!(),
            core::mem::size_of::<MultibootConsoleRequest>() as u32
        )
    };
}

macro_rules! tag_framebuffer_request {
    () => {
        tag!(
            header_tag_framebuffer!(),
            flags_framebuffer!(),
            core::mem::size_of::<MultibootFramebufferRequest>() as u32
        )
    };
}

define!(framebuffer_width, 80);
define!(framebuffer_height, 25);
define!(framebuffer_depth, 0);

macro_rules! tag_end {
    () => {
        tag!(
            header_tag_end!(),
            flags_none!(),
            core::mem::size_of::<MultibootHeaderTag>() as u32
        )
    };
}

#[link_section = ".preliminary.multiboot"]
#[no_mangle]
static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: header_magic!(),
    architecture: header_arch!(),
    header_length: header_size!(),
    checksum: header_checksum!(),
    info_request: MultibootInfoRequest {
        tag: tag_info_request!(),
        request_type: tag_type_mem_map!(),
    },
    console_request: MultibootConsoleRequest {
        tag: tag_console_request!(),
        console_flags: 3,
    },
    framebuffer_request: MultibootFramebufferRequest {
        tag: tag_framebuffer_request!(),
        width: framebuffer_width!(),
        height: framebuffer_height!(),
        depth: framebuffer_depth!(),
    },
    end_tag: tag_end!(),
};

#[repr(C, align(8))]
struct MultibootHeaderTag {
    type_: u16,
    flags: u16,
    size: u32,
}

#[repr(C)]
struct MultibootInfoRequest {
    tag: MultibootHeaderTag,
    request_type: u32,
}

#[repr(C)]
struct MultibootConsoleRequest {
    tag: MultibootHeaderTag,
    console_flags: u32,
}

#[repr(C)]
struct MultibootFramebufferRequest {
    tag: MultibootHeaderTag,
    width: u32,
    height: u32,
    depth: u32,
}

#[repr(C)]
struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    info_request: MultibootInfoRequest,
    console_request: MultibootConsoleRequest,
    framebuffer_request: MultibootFramebufferRequest,
    end_tag: MultibootHeaderTag,
}
