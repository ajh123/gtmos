// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Welcome to GT-MOS!\n").unwrap();
    vga_buffer::WRITER.lock().write_str("GT-MOS is (c) 2023 Samuel Hulme, All rights reserved.\n").unwrap();
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    loop {}
}

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
