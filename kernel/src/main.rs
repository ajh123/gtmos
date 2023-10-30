// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
// The standard library's testing implementation will not work in
// our environment so we need to define our own.
#![feature(custom_test_frameworks)]
// Specify that the `test_runner` function should be used to run
// tests.
#![test_runner(gtmos::test_runner)]
// replace the generated test main with our own
#![reexport_test_harness_main = "test_main"]

mod drivers;

use core::panic::PanicInfo;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    // println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
    // println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // use crate::drivers::vga_buffer::Colour;
    // use crate::drivers::vga_buffer::WRITER;
    // WRITER.lock().set_colour_code(Colour::White, Colour::Red);
    // print!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gtmos::test_panic_handler(info)
}
