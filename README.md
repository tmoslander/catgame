## Pluggable Interrupt Template

This project is a template that one can clone in order to set up
a [Pluggable Interrupt OS](https://crates.io/crates/pluggable_interrupt_os).

Prior to building this example, be sure to install the following:
* [Qemu](https://www.qemu.org/)
* Nightly Rust. To install:
  * `rustup default nightly`
* `llvm-tools-preview`. To install:
  * `rustup component add llvm-tools-preview`
* The [bootimage](https://github.com/rust-osdev/bootimage) tool. To install it:
  * `cargo install bootimage`