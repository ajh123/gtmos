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
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use gtmos_kernel;
use gtmos_kernel::graphics::GraphicsAPI;
use gtmos_kernel::platform::{Platform, set_platform, get_sub_system};
use gtmos_kernel_x86_64::system::X86_64SubSystem;



static mut GRAPHICS_API: Option<GraphicsAPI> = None;

bootloader_api::entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    use core::cell::RefCell;
    use gtmos_kernel::{drivers::framebuffer::Pixel, console::Console};

    let platform = Platform::new(X86_64SubSystem::new());
    set_platform(platform);

    if let Some(my_cpu) = get_sub_system() {
        if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
            let width = {framebuffer.info().width};
            let height = {framebuffer.info().height};
            let fb_mem = RefCell::new(gtmos_kernel::drivers::framebuffer::FramebufferMemory {
                width: width,
                height: height,
                bytes_per_pixel: framebuffer.info().bytes_per_pixel,
                buffer: framebuffer.buffer_mut(),
            });
            // Create graphics_api instance inside the block
            unsafe {
                GRAPHICS_API = Some(GraphicsAPI::new(fb_mem));
            }

            // Fill the entire framebuffer with a teal colour.
            unsafe {
                if let Some(api) = GRAPHICS_API.as_mut() {
                    api.draw_filled_rectangle(0, 0, width, height, Pixel {
                        b: 0x80,
                        g: 0x80,
                        r: 0x00,
                    });
                }
            }

            // Set the my_subsystem directly
            if let Some(my_subsystem) = get_sub_system() {
                unsafe {
                    let console_option = Some(Console::new(GRAPHICS_API.as_mut().unwrap(), 2));
                    my_subsystem.set_console(console_option.map(|c| c));
                }
            }
        }
        // if let Some(platform_from_get) = get_platform::<X86_64Cpu>() {
        //     // platform_from_get.console = _console;
        //     if let Some(console) = &mut platform_from_get.console {
        //         let text_colour = Pixel { r: 0xFF, g: 0xFF, b: 0xFF };
        //         let background_colour = Pixel { r: 0, g: 0x80, b: 0x80 };

        //         console.write_str("A", text_colour, background_colour);
        //         console.write_str("A\n", text_colour, background_colour);
        //         console.write_str("Agj", text_colour, background_colour);
        //     }
        // }
        if let Some(system) = get_sub_system() {
            if let Some(console) = system.get_console() {
                let text_colour = Pixel { r: 0xFF, g: 0xFF, b: 0xFF };
                let background_colour = Pixel { r: 0, g: 0x80, b: 0x80 };
                console.write_str("A", text_colour, background_colour);
                console.write_str("A\n", text_colour, background_colour);
                console.write_str("Agj", text_colour, background_colour);
            }
        }

        // gtmos_kernel::println!("AAAgj");
        gtmos_kernel::serial_println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
        // gtmos_kernel::println!("Welcome to GT-MOS!\nGT-MOS is (c) 2023 Samuel Hulme, All rights reserved.");
        gtmos_kernel::serial_println!("Hello World{}", "!");


        my_cpu.halt();
    }
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]
pub(crate) fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let platform = Platform::new(X86_64SubSystem::new());
    set_platform(platform);
    if let Some(my_cpu) = get_sub_system() {
        test_main();
        my_cpu.halt();
    }
    loop {
        x86_64::instructions::hlt();
    }
}


// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // use crate::drivers::vga_buffer::Colour;
    // use crate::drivers::vga_buffer::WRITER;
    // WRITER.lock().set_colour_code(Colour::White, Colour::Red);
    // print!("{}", info);
    x86_64::instructions::interrupts::disable();
    gtmos_kernel::serial_println!("{}", info);
    loop {
        x86_64::instructions::interrupts::disable();
        x86_64::instructions::hlt();
    }
}
