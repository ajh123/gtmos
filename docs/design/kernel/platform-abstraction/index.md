# GT-MOS Kernel

The kernel shall be divided into multiple crates / modules:

- Main "core" located at [../../gtmos_kernel/](../../gtmos_kernel/).
- x86_64 support located at [../../gtmos_kernel_x86_64/](../../gtmos_kernel_x86_64/).

These crates / modules will allow support for multiple CPU architectures in the future.

If you can open draw.io files then you may download [./diagram.drawio](./diagram.drawio) for more implementation details.

## Main "core" crate

This will contain APIs and abstractions for everything the kernel should have, for example the
[device abstraction layer](./device-manager/index.md).

## x86_64 support crate / module

This crate / module will have all supporting functionality for running the kernel on x86_64 platforms.
