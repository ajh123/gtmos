// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;
use crate::vga_buffer::Colour;
use crate::vga_buffer::WRITER;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
    println!("Hello World{}", "!");
    panic!("Some panic message");
    // loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    WRITER.lock().set_colour_code(Colour::White, Colour::Red);
    print!("{}", info);
    loop {}
}
