use crate::platform::get_platform;

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    if let Some(platform) = get_platform() {
        let message = args.as_str().unwrap_or_default();
        platform.write("serial", message);
    }
}

// pub fn receive() -> char  {
//     let char1 = SERIAL1.lock().receive() as char;
//     return char1;
// }

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
/// Prints a message with a new line to Serial. Use this exactly like the `println` macro from the Rust
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


#[test_case]
/// Just prints something to the VGA buffer
fn test_println_simple() {
    serial_println!("test_println_simple output");
}
