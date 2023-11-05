#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gtmos_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

// use gtmos::println;
use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gtmos_kernel::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    // println!("test_println output");
}
