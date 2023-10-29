# Running <!-- omit in toc -->

Once you have [built](./building.md) the os, there are many ways to run it:

- [QEMU with raw image](#qemu-with-raw-image)
- [Cargo run](#cargo-run)
- [Cargo test](#cargo-test)
- [Real machine](#real-machine)

## QEMU with raw image

You may run the image with `qemu-system-x86_64 -drive format=raw,file=target/x86_64-gtmos/debug/bootimage-gtmos.bin`.

## Cargo run

You may use `cargo run` to run the image. This will open QEMU in a similar way to before.

> [!NOTE]  
> Bonus: `cargo run` will build the os to!

## Cargo test

You may use `cargo test` to run the image like in [`cargo run`](#cargo-run) but it will run tests instead.

## Real machine

On linux machine the command below may be used to write the image to a disk.

> [!WARNING]  
> Be careful because **this will erase all data** on the disk!

`dd if=target/x86_64-gtmos/debug/bootimage-gtmos.bin of=/dev/sdX && sync`

You must replace the `sdX` with the device name of your disk. You may find them with `lsblk` (list block devices).
