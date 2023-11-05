// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]
// The standard library's testing implementation will not work in
// our environment so we need to define our own.
#![feature(custom_test_frameworks)]
// Specify that the `test_runner` function should be used to run
// tests.
#![test_runner(gtmos_kernel::test_runner)]
// replace the generated test main with our own
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

bootloader_api::entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    use core::cell::RefCell;
    use gtmos_kernel::drivers::framebuffer::Pixel;
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let width = {framebuffer.info().width};
        let height = {framebuffer.info().height};
        let fb_mem = RefCell::new(gtmos_kernel::drivers::framebuffer::FramebufferMemory {
            width: width,
            height: height,
            bytes_per_pixel: framebuffer.info().bytes_per_pixel,
            buffer: framebuffer.buffer_mut(),
        });
        let graphics_api: &mut gtmos_kernel::graphics::GraphicsAPI = &mut gtmos_kernel::graphics::GraphicsAPI::new(fb_mem);

        // Fill the entire framebuffer with a teal colour.
        graphics_api.draw_filled_rectangle(0, 0, width, height, Pixel {
            b: 0x80,
            g: 0x80,
            r: 0x00
        });

        // Draw a line from (10, 10) to (100, 100) with a red colour.
        graphics_api.draw_line(10, 10, 100, 100, Pixel {
            b: 0x00,
            g: 0x00,
            r: 0xFF
        });

        // Draw a filled rectangle at (50, 50) with a blue colour.
        graphics_api.draw_filled_rectangle(5, 5, 30, 30, Pixel {
            b: 0xFF,
            g: 0x00,
            r: 0x00
        });

        graphics_api.draw_char_transparent(
            0,
            0,
            'A',
            Pixel {
                b: 0xFF,
                g: 0xFF,
                r: 0xFF
            },
            1
        );

        graphics_api.draw_char_transparent(
            0,
            0,
            'A',
            Pixel {
                b: 0xFF,
                g: 0xFF,
                r: 0xFF
            },
            25
        );

        graphics_api.draw_char_transparent(
            80,
            80,
            'A',
            Pixel {
                b: 0xFF,
                g: 0xFF,
                r: 0xFF
            },
            50
        );
    }

    gtmos_kernel::serial_println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
    gtmos_kernel::serial_println!("Hello World{}", "!");

    loop {
        let c = gtmos_kernel::drivers::serial::receive();
        gtmos_kernel::serial_print!("{}", c);
    }
}

#[cfg(test)]
pub(crate) fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
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
    gtmos_kernel::serial_println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gtmos_kernel::test_panic_handler(info);
}
