#![no_std]
#![no_main]
#![cfg_attr(feature = "panic_message", feature(panic_info_message))]
#![feature(alloc_error_handler)]

mod debug;
use debug::exit;

mod memory;

#[macro_use(format)]
extern crate alloc;
use alloc::{string::String, string::ToString};

#[link_section = ".init"]
#[no_mangle]
unsafe extern "C" fn start() {
    memory::init(HEAP_START, HEAP_SIZE);
    main();
    exit(0)
}

#[no_mangle]
fn main() {
    print!("\n[Example of embedded development in Rust]\n\n");

    // Now that we have unleashed memory allocation, we can use String and format!
    let s = format!("{:08X}", 305419896);
    println!("s = {}", "'".to_string() + &s + &"'");
    println!();

    dbg!(fibs("A".repeat(10)).len());
    println!()
}

fn fibs(s : String) -> String {
   match &*s {
    "A" | "" =>  s,
    _ =>  [ fibs(s[1..].to_string()), fibs(s[2..].to_string()) ].concat()
    }
}

#[allow(dead_code)]
#[no_mangle]
fn fibf(n : f64) -> f64 {
    if n < 2. { n } else { fibf(n-1.) + fibf(n-2.) }
}

// memory layout
const UART_BASE: usize = 0x1000_0000;
const UART_THR: usize = UART_BASE + 0;
const HEAP_START: usize = 0x8008_0000;
const HEAP_SIZE: usize = 0x0004_0000; // in bytes
