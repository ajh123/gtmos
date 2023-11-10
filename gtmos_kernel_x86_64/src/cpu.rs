use crate::{interrupts, gdt};

pub fn initialise() {
    gdt::initialise();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() }
}
