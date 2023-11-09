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

pub mod interrupts;
pub mod cpu;
pub mod gdt;
