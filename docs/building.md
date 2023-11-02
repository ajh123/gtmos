# Building image file

The following command need to be done OUTSIDE this repository (for example your home folder):
`rustup target add x86_64-unknown-none`

Then navigate back to this repository and run:
`cargo build`

For a release build run:
`cargo build --release`

If you get any errors please go to [Troubleshooting](#troubleshooting).

For normal builds the resulting image files will be located at:

* UEFI disk image at `/target/debug/uefi.img`
* BIOS disk image at `/target/debug/bios.img`

A release build will have `debug` replaced with `release`.

> [!NOTE]  
> If you don't see the image files in their path you may need to run `cargo run  --bin gtmos`. This will run `cargo build` automatically.

You now may [run the os](./running.md)!

> [!NOTE]  
> Bonus: `cargo run` will build the os to!

## Troubleshooting

### Error: Kernel build failed

> [!NOTE]  
> This may not be relevant to the UEFI rewrite of GT-MOS.

If you get an error like:

```text
error: "/home/samro/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/Cargo.lock" does not exist, unable to build with the standard library, try:
        rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
Error: Kernel build failed.
Stderr:
```

run the command it gives you, in this example `rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu`

Then run `cargo bootimage` again.

### Error: Bootloader build failed

> [!NOTE]  
> This may not be relevant to the UEFI rewrite of GT-MOS.

If you get an error like:

```text
Caused by:
  process didn't exit successfully: `F:/rust/gtmos/target/bootimage/bootloader/release/build/bootloader-d6ed16076e08b61f/build-script-build` (exit code: 1)
  --- stderr
  Error: llvm-tools not found
  Maybe the rustup component `llvm-tools-preview` is missing?
    Install it through: `rustup component add llvm-tools-preview`
warning: build failed, waiting for other jobs to finish...
Error: Bootloader build failed.
Stderr:
```

run the command it gives you, in this example `rustup component add llvm-tools-preview`

Then run `cargo bootimage` again.
