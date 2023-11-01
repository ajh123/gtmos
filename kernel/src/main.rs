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
mod graphics;

use core::{panic::PanicInfo, cell::RefCell};

use drivers::framebuffer::Pixel;

bootloader_api::entry_point!(kernel_main);


fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {

    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let fb_mem = RefCell::new(drivers::framebuffer::FramebufferMemory {
            width: framebuffer.info().width,
            height: framebuffer.info().height,
            bytes_per_pixel: framebuffer.info().bytes_per_pixel,
            buffer: framebuffer.buffer_mut(),
        });
        let graphics_api: &mut graphics::GraphicsAPI = &mut graphics::GraphicsAPI::new(fb_mem);

        // drivers::framebuffer::Framebuffer::fill(fb, drivers::framebuffer::Pixel {
        //     b: 0xFF,
        //     g: 0xFF,
        //     r: 0xFF,
        //     channel: 0
        // });

        // Fill the entire framebuffer with a white colour.
        // graphics_api.draw_filled_rectangle(0, 0, graphics_api.get_width(), graphics_api.get_height(), Pixel {
        //     b: 0xFF,
        //     g: 0xFF,
        //     r: 0xFF,
        //     channel: 0xFF,
        // });

        // Fill the entire framebuffer with a white colour.
        // graphics_api.fill_framebuffer(Pixel {
        //     b: 0xFF,
        //     g: 0xFF,
        //     r: 0xFF,
        //     channel: 0xFF,
        // });

        // Draw a line from (10, 10) to (100, 100) with a red colour.
        let red_pixel = Pixel {
            b: 0x00,
            g: 0x00,
            r: 0xFF,
            channel: 0x00,
        };
        graphics_api.draw_line(10, 10, 100, 100, red_pixel);

        // Draw a filled rectangle at (50, 50) with a blue colour.
        let blue_pixel = Pixel {
            b: 0xFF,
            g: 0x00,
            r: 0x00,
            channel: 0x00,
        };
        graphics_api.draw_filled_rectangle(5, 5, 30, 30, blue_pixel);
    }

    // println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
    // println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

pub fn main() {}

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
