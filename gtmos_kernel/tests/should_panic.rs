#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gtmos_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
// use gtmos::{QemuExitCode, exit_qemu, serial_println};
// use gtmos::serial_print;

#[no_mangle]
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    should_fail();
    // serial_println!("[test did not panic]");
    // exit_qemu(QemuExitCode::Failed);
    loop{}
}

fn should_fail() {
    // serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // serial_println!("[ok]");
    // exit_qemu(QemuExitCode::Success);
    loop {}
}
