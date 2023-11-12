use gtmos_kernel::{platform::SubSystem, drivers::framebuffer::Pixel, console::Console};
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


pub struct X86_64SubSystem {
    pub console: Option<Console<'static>>,
}

impl X86_64SubSystem {
    pub fn new() -> Self {
        let system = X86_64SubSystem { console: None };
        system.initialise();
        return system;
    }
}

impl SubSystem for X86_64SubSystem {
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

    fn write(&mut self, dest: &str, data: &str) {
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
            let text_colour = Pixel { r: 0xFF, g: 0xFF, b: 0xFF };
            let background_colour = Pixel { r: 0, g: 0x80, b: 0x80 };
            if let Some(console) = self.get_console() {
                interrupts::without_interrupts(|| {
                console.write_str(data, text_colour, background_colour);
                });
            }
        }
    }

    // fn read(&self, dest: &str) -> &str {
    //     return "";
    // }


    fn set_console(&mut self, console: Option<Console<'static>>) {
        self.console = console;
    }

    fn get_console(&mut self) -> Option<&mut Console<'static>> {
        return self.console.as_mut()
    }

}
