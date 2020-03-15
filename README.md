# Examples of embedded application in minimalistic style for RISC-V in Rust

## How to build

* Install Rust  following instructions on [this page](https://www.rust-lang.org/tools/install)

* rustup +nightly component add rust-src

* rustup +nightly target add riscv64gc-unknown-none-elf

* cargo install cargo-xbuild 

* cargo +nightly xbuild --release

### optionally

  * rustup +stable component add llvm-tools-preview

  * cargo install cargo-binutils

then this will work.

> rustup run stable rust-objcopy -O binary target/riscv64gc-unknown-none-elf/release/step1-puts step1-puts.text

or you may use gnu binutils' objdump or llvm-objdmp with proper riscv support from your distributyon or elsewhere.

for instance you can find riscv64-unknown-elf toolchain [here](https://github.com/kendryte/kendryte-gnu-toolchain/releases).

## Other notes

* link.x includes a few assembler instructions to setup the stack pointer and the program counter.

* Use [rvemu](https://crates.io/crates/rvemu) to run the generated codes.
