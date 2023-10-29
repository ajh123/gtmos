// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
// The standard library's testing implementation will not work in
// our environment so we need to define our own.
#![feature(custom_test_frameworks)]
// Specify that the `test_runner` function should be used to run
// tests.
#![test_runner(crate::test_runner)]
// replace the generated test main with our own
#![reexport_test_harness_main = "test_main"]

mod drivers;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::drivers::vga_buffer::Colour;
    use crate::drivers::vga_buffer::WRITER;
    WRITER.lock().set_colour_code(Colour::White, Colour::Red);
    print!("{}", info);
    loop {}
}

// Testing implementation

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::drivers::qemu;
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit_qemu(qemu::QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]

pub fn test_runner(tests: &[&dyn Testable]) {
    use crate::drivers::qemu;
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    qemu::exit_qemu(qemu::QemuExitCode::Success);
}
