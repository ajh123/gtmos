//! Module containing drivers for hardware connected to the computer.
//!
//! Divers are essential to make sure it is easy / simple to use the hardware, otherwise you will need
//! to directly access the hardware, through I/O ports or Memory Mapped I/O, which can be difficult and
//! increase the chances for errors. Drivers abstract this complexity away, into functions which do the
//! work for you.
//!
//! ## See also:
//! * [Port IO (OsDev.org)](https://wiki.osdev.org/Port_IO)
//! * [MemoryMap (x86) (OsDev.org)](https://wiki.osdev.org/Memory_Map_(x86))

// pub mod serial;
// pub mod vga_buffer;
