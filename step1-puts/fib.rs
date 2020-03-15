#![no_std]
#![no_main]
#![cfg_attr(feature = "panic_message", feature(panic_info_message))]

const MSG: &str = r#"
[Example of embedded development in Rust]

See the value of a0 register, that is x10 in numeric.

"#;

#[link_section = ".init"]
#[no_mangle]
unsafe extern "C" fn start() {
    main();
    exit(0)
}

#[no_mangle]
fn main() {
    puts(MSG);
    let val = fib(10);
    exit(val)
}

fn fib(n : i32) -> i32 {
    match n {
    0 | 1 => n,
    _ => fib(n-1) + fib(n-2)
    }
}

fn putc(c: u8) -> () {
    const UART_BASE: usize = 0x1000_0000;
    const UART_THR: usize = UART_BASE + 0;
    unsafe { core::ptr::write_volatile(UART_THR as *mut u8, c); }
}

#[inline(never)]
#[no_mangle]
pub fn puts(s: &str) -> () {
    for c in s.bytes() {
        putc(c);
    }
}

#[no_mangle]
pub fn exit(code: i32) -> ! {
    extern { fn _exit(_: i32) -> !; }
    unsafe { _exit(code); }
}

// compiler requires this exist even when this is not used
#[inline(never)]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    exit(-1)
}
