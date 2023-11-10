use crate::{interrupts, gdt};

pub fn initialise() {
    gdt::initialise();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }
    x86_64::instructions::interrupts::enable();
}
