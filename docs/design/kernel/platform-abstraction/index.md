# GT-MOS Kernel

The kernel shall be used on top of a "[SubSystem](#subsystems)".

If you can open draw.io files then you may download [./diagram.drawio](./diagram.drawio) for more implementation details. Also you may see [./layers.drawio](./layers.drawio) to see how different parts of GT-MOS interact with each other.

## Main "core" crate

This will contain APIs and abstractions for everything the kernel should have, for example the
[device abstraction layer](./device-manager/index.md).

## SubSystems

A SubSystem is a platform dependant API to be used by the kernel. All operations done by the kernel will go through the current SubSystem. For example [x86_64 systems](../../gtmos_kernel_x86_64/) will get their own SubSystem.
