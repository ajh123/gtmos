//! # GT-MOS
//! GT-MOS is a simple operating system for the x86_64 architecture. It will contain a lot of useful
//! features (which are not yet implemented).
//!
//! ## Building GT-MOS
//! Please see the guides located at [Building](./../docs/building.md) and [Running](./../docs/running.md).
//!

#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod drivers;
pub mod graphics;

use core::panic::PanicInfo;


#[doc(hidden)]
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[doc(hidden)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("dsfuidhdfshidfhidfuidfh");
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[doc(hidden)]
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}


#[cfg(test)]
bootloader_api::entry_point!(kernel_main);
/// Entry point for `cargo test`
#[cfg(test)]
pub(crate) fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[doc(hidden)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[doc(hidden)]
pub fn exit_qemu(exit_code: QemuExitCode) {
    // use x86_64::instructions::port::Port;

    // unsafe {
    //     let mut port = Port::new(0xf4);
    //     port.write(exit_code as u32);
    // }
}
