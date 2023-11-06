# GT-MOS development roadmap

## Kernel

- [WIP] x86_64 support
  - [x] UEFI / BIOS booting: @ajh123: [commit 3a713e2](https://github.com/ajh123/gtmos/commit/3a713e2abe42f1b001dff9ace36e04ac082ba608)
  - [x] Framebuffer driver: @ajh123: [commit e9e5976](https://github.com/ajh123/gtmos/commit/e9e5976fd597652ca95d79ba37c09af1f4ef6cda)
  - [x] Basic serial drivers: @ajh123: [commit d142ca3](https://github.com/ajh123/gtmos/commit/d142ca3c5048d79223c5b4d556e688799f584382)
  - [ ] Get `cargo test` to work again
  - [ ] Memory management systems
  - [ ] Interrupts
  - [ ] More input methods (keyboard / mouse)
  - [ ] Platform abstraction support
  - [ ] Device system implementation (with support for platform independent device abstraction)
    - [ ] USB drivers
    - [ ] Better serial / framebuffer / input drivers
  - [ ] Multitasking (with support for async / await)
  - [ ] UNIX / POSIX compatible System calls
  - [ ] Userland with ELF program loader
  - [ ] Some kind of system library for user land
- [WIP] Platform independency
  - [ ] Platform abstraction
  - [ ] Device abstraction e.g. <https://github.com/rust-embedded/embedded-hal/>
  - [x] Basic graphics abstraction: @ajh123
    - [x] Pixel / Line / Rectangle drawing: @ajh123: [commit 788ce62](https://github.com/ajh123/gtmos/commit/788ce62a87d4ca7fa0467b9da0b8db5c6c1ef14a)
    - [x] Basic text rendering: @ajh123: [commit 4ecba3f](https://github.com/ajh123/gtmos/commit/4ecba3f3625e78a542e37bb9149ac9ad112c0a56)
  - [ ] Proper graphics abstraction
