# GT-MOS Kernel

The kernel shall be divided into multiple crates / modules:

- Main "core" located at [../../kernel/](../../kernel/).
- x86_64 support located at [../../kernel/x86_64/](../../kernel/x86_64/).

These crates / modules will allow support for multiple CPU architectures in the future.

## Main "core" crate

This will contain APIs and abstractions for everything the kernel should have, for example the
[device abstraction layer](./device-manager/index.md).

## x86_64 support crate / module

This crate / module will have all supporting functionality for running the kernel on x86_64 platforms.
