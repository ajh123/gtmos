// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
// The standard library's testing implementation will not work in
// our environment so we need to define our own.
#![feature(custom_test_frameworks)]
// Specify that the `test_runner` function should be used to run
// tests.
#![test_runner(kernel::test_runner)]
// replace the generated test main with our own
#![reexport_test_harness_main = "test_main"]

use core::{panic::PanicInfo, cell::RefCell};

use kernel::drivers::framebuffer::Pixel;

bootloader_api::entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let width = {framebuffer.info().width};
        let height = {framebuffer.info().height};
        let fb_mem = RefCell::new(kernel::drivers::framebuffer::FramebufferMemory {
            width: width,
            height: height,
            bytes_per_pixel: framebuffer.info().bytes_per_pixel,
            buffer: framebuffer.buffer_mut(),
        });
        let graphics_api: &mut kernel::graphics::GraphicsAPI = &mut kernel::graphics::GraphicsAPI::new(fb_mem);

        // Fill the entire framebuffer with a white colour.
        graphics_api.draw_filled_rectangle(0, 0, width+1, height+1, Pixel {
            b: 0xFF,
            g: 0xFF,
            r: 0xFF,
            channel: 0xFF,
        });

        // Draw a line from (10, 10) to (100, 100) with a red colour.
        graphics_api.draw_line(10, 10, 100, 100, Pixel {
            b: 0x00,
            g: 0x00,
            r: 0xFF,
            channel: 0xFF,
        });

        // Draw a filled rectangle at (50, 50) with a blue colour.
        graphics_api.draw_filled_rectangle(5, 5, 30, 30, Pixel {
            b: 0xFF,
            g: 0x00,
            r: 0x00,
            channel: 0xFF,
        });
    }

    kernel::serial_println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
    kernel::serial_println!("Hello World{}", "!");

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
    kernel::serial_println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}
