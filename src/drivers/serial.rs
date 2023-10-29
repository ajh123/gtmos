use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    /// This is a global (static) instance of a [`SerialPort`] struct. It is used to write to serial.
    /// You may use the macros [`serial_print!`](../../macro.serial_print.html) and
    /// [`serial_println!`](../../macro.serial_println.html) to use it.
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
/// Prints a message to Serial. Use this exactly like the `print` macro from the Rust standard library.
///
/// ## Example
/// ```rust
/// serial_print!("Hello, world!");
/// ```
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::drivers::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
/// Prints a message with a new line to Serial. Use this exactly like the `print` macro from the Rust
/// standard library.
///
/// ## Example
/// ```rust
/// serial_println!("Hello, world!");
/// ```
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}
