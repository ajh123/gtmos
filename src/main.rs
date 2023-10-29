// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
    println!("Hello World{}", "!");
    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
