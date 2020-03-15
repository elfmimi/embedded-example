#![no_std]
#![no_main]
#![cfg_attr(feature = "panic_message", feature(panic_info_message))]

mod debug;
use debug::exit;

#[link_section = ".init"]
#[no_mangle]
unsafe extern "C" fn start() {
    crate::main();
    exit(0)
}

#[no_mangle]
fn main() {
    print!(concat!("\n", "[Example of embedded development in Rust]\n", "\n"));

    for i in 1..=10 {
        let val = fib(i);
        println!("fib({i}) = {val}", val = val, i = i)
    }
    println!();

    dbg!(fib(11));
    println!()
}

fn fib(n : i32) -> i32 {
    match n {
    0 | 1 => n,
    _ => fib(n-1) + fib(n-2)
    }
}

// memory layout
const UART_BASE: usize = 0x1000_0000;
const UART_THR: usize = UART_BASE + 0;
