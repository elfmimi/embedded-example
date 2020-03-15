#!/bin/bash
OBJCOPY="rustup run stable rust-objcopy"
OBJDUMP="rustup run stable rust-objdump"
# objdump from gnu binutils gives more comprehensive results.
# OBJDUMP=riscv64-unknown-elf-objdump

case $1 in

clean)
echo clean
rm -rf $(dirname $0)/target *.text
;;

rustc)
# This simple single file example can be compiled with only the stable rustc (riscv64gc without c).
# rustc +stable step1-puts/fib.rs --target riscv64gc-unknown-none-elf -C target-feature=-c -C codegen-units=1 -C opt-level=s -C lto -C force_frame_pointers=no -C link-arg=--defsym=_exit=4000000000 -C link-arg=--defsym=abort=4040404040 -C link-arg=--image-base=0x80000000 -C link-arg=--section-start=.init=0x80000000 -C link-arg=--section-start=.text=0x80000020 -C link-arg=--section-start=.rodata=0x80000100 -C link-arg=--entry=start -o fib.elf
rustc +stable step1-puts/fib.rs --target riscv64gc-unknown-none-elf -C target-feature=-c -C codegen-units=1 -C opt-level=s -C lto -C force_frame_pointers=no -C link-arg=-Tstep1-puts/link.x -o fib.elf
;;

install-binutils) # install rust-objcopy , rust-objdump and others
rustup +nightly component add llvm-tools-preview
cargo +nightly install cargo-binutils
;;

objcopy)
($0) &&
${OBJCOPY} -O binary target/riscv64gc-unknown-none-elf/release/step1-puts step1-puts.text &&
${OBJCOPY} -O binary target/riscv64gc-unknown-none-elf/release/step2-print step2-print.text &&
${OBJCOPY} -O binary target/riscv64gc-unknown-none-elf/release/step3-string step3-string.text
;;

objdump)
${OBJDUMP} -d -C target/riscv64gc-unknown-none-elf/release/step1-puts | less
;;

*)
cargo +nightly xbuild --release
;;

esac
