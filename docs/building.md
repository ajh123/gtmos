# Building image file

The following command need to be done OUTSIDE this repository (for example your home folder):
`cargo install bootimage`

Then navigate back to this repoistory and run:
`cargo bootimage`

If you get any errors please go to [Troubleshooting](#troubleshooting).

The resulting image file will be located in `target/x86_64-gtmos/debug/bootimage-gtmos.bin`

You now may [run the os](./running.md)!

> [!NOTE]  
> Bonus: `cargo run` will build the os to!

## Troubleshooting

### Error: Kernel build failed

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

If you get an error like:

```text
Caused by:
  process didn't exit successfully: `F:\rust\gtmos\target\bootimage\bootloader\release\build\bootloader-d6ed16076e08b61f\build-script-build` (exit code: 1)
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
