use gtmos_kernel::{platform::{CPU, get_platform}, drivers::framebuffer::Pixel};
use uart_16550::SerialPort;
use spin::Mutex;
use crate::{interrupts, gdt};
use lazy_static::lazy_static;

lazy_static! {
    /// This is a global (static) instance of a [`SerialPort`] struct. It is used to write to serial.
    /// You may use the macros [`serial_print!`](../../macro.serial_print.html) and
    /// [`serial_println!`](../../macro.serial_println.html) to use it.
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}


pub struct X86_64Cpu;

impl CPU for X86_64Cpu {
    fn initialise(&self) {
        gdt::initialise();
        interrupts::init_idt();
        unsafe { interrupts::PICS.lock().initialize() }
        x86_64::instructions::interrupts::enable();
    }

    fn halt(&self) {
        loop {
            x86_64::instructions::hlt();
        }
    }

    fn write(&self, dest: &str, data: &str) {
        use x86_64::instructions::interrupts;
        if dest == "serial" {
            use core::fmt::Write;

            interrupts::without_interrupts(|| {
                SERIAL1
                    .lock()
                    .write_str(data)
                    .expect("Printing to serial failed");
            });
        }
        if dest == "vga_console" {
            interrupts::without_interrupts(|| {
                if let Some(platform_from_get) = get_platform::<X86_64Cpu>() {
                    if let Some(console) = &mut platform_from_get.console {
                        let text_colour = Pixel { r: 0xFF, g: 0xFF, b: 0xFF };
                        let background_colour = Pixel { r: 0, g: 0x80, b: 0x80 };
                        console.write_str(data, text_colour, background_colour);
                    }
                }
            });
        }
    }

    // fn read(&self, dest: &str) -> &str {
    //     return "";
    // }
}
