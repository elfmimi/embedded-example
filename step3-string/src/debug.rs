use crate::UART_THR;

fn putc(c: u8) -> () {
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

#[cfg(feature = "abort_handler")]
#[export_name = "abort"]
fn abort() -> ! {
    puts("abort!\n");
    exit(-1)
}

#[cold]
#[inline(never)]
pub fn dprint_fmt(fmt: core::fmt::Arguments<'_>) -> () {
    use core::fmt::Write;

    struct DprintFormatter;

    impl core::fmt::Write for DprintFormatter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            puts(s);
            Ok(())
        }
    }

    let mut formatter = DprintFormatter;
    formatter.write_fmt(fmt).ok();
}

#[macro_export]
macro_rules! print {
    ($msg:expr) => {
        $crate::debug::puts($msg)
    };
    ($msg:expr,) => {
        $crate::print!($msg)
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::debug::dprint_fmt(format_args!($fmt, $($arg)+))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::print!(concat!($fmt, "\n"), $($arg)+)
    };
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", core::file!(), core::line!());
    };
    ($val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    core::file!(), core::line!(), core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($val:expr,) => { $crate::dbg!($val) };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}

// compiler requires this exist even when this is not used
#[inline(never)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    #[cfg(not(feature = "panic_location"))]
    println!("panic occurred but no location information");

    #[cfg(feature = "panic_location")]
    {
        if let Some(location) = _info.location() {
            println!("panic occurred in file '{}' at line {}", location.file(),
                location.line());
        } else {
            println!("panic occurred but can't get location information...");
        }
    }

    #[cfg(feature = "panic_message")]
    {
        if let Some(msg) = _info.message() {
            println!("panic_info_message: \"{}\"", msg);
            // dprint_fmt(format_args!("panic_info_message: \"{}\"\n", msg));
        }
    }

    exit(-1)
}
