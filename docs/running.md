# Running <!-- omit in toc -->

Once you have [built](./building.md) the os, there are many ways to run it:

- [QEMU](#qemu)
  - [BIOS](#bios)
  - [UEFI](#uefi)
  - [Cargo run](#cargo-run)
- [Cargo test](#cargo-test)
- [Real machine](#real-machine)

## QEMU

### BIOS

You may run the image with `qemu-system-x86_64 -drive format=raw,file=target/debug/bios.img`.
A release build will have `debug` replaced with `release`.

Additionally, you may use `cargo run --bin qemu-bios` to run the image.

> [!NOTE]
> If you don't see the image files in their path you may need to run `cargo run  --bin gtmos_tools`. This will run `cargo build` automatically.

### UEFI

You may run the image with `qemu-system-x86_64 -drive format=raw,file=target/debug/uefi.img -bios OVMF-pure-efi.fd`.
A release build will have `debug` replaced with `release`.

> [!NOTE]
> This assumes you have a copy of `OVMF-pure-efi.fd`.

Additionally, you may use `cargo run --bin qemu-uefi` to run the image. This will have `OVMF-pure-efi.fd` included!

> [!NOTE]
> If you don't see the image files in their path you may need to run `cargo run  --bin gtmos_tools`. This will run `cargo build` automatically.

### Cargo run

You may use `cargo run` to run the image. By default it will open QEMU just like [QEMU in BIOS](#bios)

> [!NOTE]
> Bonus: `cargo run` will build the os to!
>
> If you don't see the image files in their path you may need to run `cargo run  --bin gtmos_tools`. This will run `cargo build` automatically.

## Cargo test

You may use `cargo test` to run the image like in [`cargo run`](#cargo-run) but it will run tests instead.

> [!WARNING]
> Testing system is not implemented! See https://github.com/ajh123/gtmos/issues/6 for more details.

## Real machine

On linux machine the command below may be used to write the image to a disk.

> [!WARNING]
> Be careful because **this will erase all data** on the disk!

- For UEFI machines: `dd if=target/debug/uefi.img of=/dev/sdX && sync`
- For BIOS machines: `dd if=target/debug/bios.img of=/dev/sdX && sync`

A release build will have `debug` replaced with `release`.

You must replace the `sdX` with the device name of your disk. You may find them with `lsblk` (list block devices).
