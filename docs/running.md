# Running <!-- omit in toc -->

Once you have [built](./building.md) the os, there are many ways to run it:

- [QEMU with raw image](#qemu-with-raw-image)
- [Cargo run](#cargo-run)
- [Real machine](#real-machine)

## QEMU with raw image

You may run the image with `qemu-system-x86_64 -drive format=raw,file=target/x86_64-gtmos/debug/bootimage-gtmos.bin`.

## Cargo run

You may use `cargo run` to run the image. This will open QEMU in a similar way to before.

Bonus: `cargo run` will build the os to!

## Real machine

On linux machine the command below may be used to write the image to a disk.
Be careful because **this will erase all data** on the disk!

`dd if=target/x86_64-gtmos/debug/bootimage-gtmos.bin of=/dev/sdX && sync`

You must replace the `sdX` with the device name of your disk. You may find them with `lsblk` (list block devices).
