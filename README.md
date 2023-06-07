<div align="center">
    <img src="static/cover.png" width="960">
</div>

<div align="center">
    <img src="https://img.shields.io/badge/QEMU-%232196F3?style=for-the-badge&logo=qemu&logoColor=white&labelColor=063760">
    <img src="https://img.shields.io/badge/Assembly-%23FFC107?style=for-the-badge&logo=academia&logoColor=white&labelColor=664d00">
    <img src="https://img.shields.io/badge/Rust-%23903C50?style=for-the-badge&logo=rust&logoColor=white&labelColor=481e28">
</div>

<br>

<div align="center">
    <a href="https://github.com/quaternotech/asmos/actions/workflows/rust.yml">
        <img src="https://github.com/quaternotech/asmos/actions/workflows/rust.yml/badge.svg" alt="Rust">
    </a>
    <a href="LICENSE.md">
        <img src="https://img.shields.io/badge/License-MIT-%23007396?labelColor=2b3137">
    </a>
</div>

<br>

<p align="justify">
<b>asmOS</b> is a modular operating system with a microkernel architecture that allows for easy customization and
extension. The operating system aims to provide a small set of essential services, such as memory management, process
scheduling, and interrupt handling. Additional services can be added through modules, which can be loaded and unloaded
dynamically.
</p>

<p align="justify">
This is a private project intended for experimentation and exploration in the field of low-level system programming.
While it is designed to be lightweight and customizable, it is not intended for production use. The microkernel
architecture and modular design provide an excellent platform for learning and tinkering with operating systems.
</p>

## Features

- Modular design with a microkernel architecture.
- Rust-based implementation for reliability and safety.
- As of now, only <b>x86_64</b> architecture is supported.

## Getting Started

To build and run <b>asmOS</b>, you will need:

- Rustc, a compiler for Rust, the nightly channel (`>= 22-02-2023`)
- Cargo, a package manager for Rust.
- QEMU, an emulation system.

Kindly note that running <b>asmOS</b> on bare-metal is not recommended.

### Building and Running

1. Clone the repository and navigate to the project directory.
2. To build the project.

   ```shell
   cargo build --release
   ```

3. To run the operating system in QEMU.

   ```shell
   cargo run --release
   ```

## Author

Mansoor Ahmed Memon

- https://github.com/mansoormemon

## Links

- [Repository](https://github.com/quaternotech/asmos)

## License

This project is licensed under the [Apache License 2.0](LICENSE.md).

<br>

<p align="center">
   <sub><sup>Copyright 2023 © Quaterno LLC</sup></sub>
</p>
