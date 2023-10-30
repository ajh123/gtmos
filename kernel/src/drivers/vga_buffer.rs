use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// suppress warning for each unused variant
#[allow(dead_code)]
// enable copy semantics for the type and make it printable and comparable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// store each member as 8 bits (a [u8]). 4 bits would be more efficient but Rust
// does not have a `[u4]`.
#[repr(u8)]
/// Represents each VGA colour. Normally it is used with [`Writer::set_colour_code`].
///
/// ## Example
/// ```rust
/// use crate::drivers::vga_buffer::WRITER;
/// WRITER.lock().set_colour_code(Colour::White, Colour::Red);
/// ```
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// make sure the same data type as Colour is used.
#[repr(transparent)]
/// ColourCode represents a combination of foreground and background [`Colour`]s.
///
/// ## Example
/// ```rust
/// let colour_code = ColourCode::new(Colour::Red, Colour::Green);
/// ```
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// make sure this struct is laid out the same as a C struct.
#[repr(C)]
/// Represents a character on the screen with a [`ColourCode`].
///
/// ## Example
/// ```rust
/// let colour_code = ColourCode::new(Colour::Red, Colour::Green);
/// let s_char = ScreenChar {
///     ascii_character: 'a',
///     colour_code,
/// });
/// ```
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

// 25 rows
const BUFFER_HEIGHT: usize = 25;
// 80 columns
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
/// Buffer is a 2D array of [`ScreenChar`]s, it represents the screen.
struct Buffer {
    // this must be Volatile because rust does not know this is memory mapped IO
    // for VGA and rust will try to optimise it because it does not know the memory
    // will be read by the graphics hardware.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Writer is a wrapper around the VGA buffer, it keeps track of the current foreground / background
/// [`Colour`]s and the last row's column position. Instead of creating your own [`Writer`], use the
/// static global instance (`WRITER`), as seen in the examples.
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    // use static to make sure this value exists forever.
    buffer: &'static mut Buffer,
}

impl Writer {
    /// Writes a single byte to the screen. The colours must be set using [`Writer::set_colour_code`].
    /// The byte must be a printable ASCII [`char`]. The "Code Page 437" character set is the default
    /// for VGA, so, no special unicode characters are supported. Code Page 437 contains ASCII, so
    /// standard ASCII characters will work. In the future more character sets will be supported.
    ///
    /// ## Example
    /// ```rust
    /// use crate::drivers::vga_buffer::WRITER;
    /// WRITER.lock().write_byte(b'H');
    /// ```
    /// In this example, the character "H" (represented by `b'H'`) is written to the screen.
    /// The character is drawn using the current foreground / background [`Colour`]s.
    ///
    /// ## See also:
    /// * [Code Page 437 (Wikipedia)](https://en.wikipedia.org/wiki/Code_page_437)
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let colour_code = self.colour_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    colour_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Writes a string to the screen. The colours must be set using [`Writer::set_colour_code`].
    /// This function is implemented using [`Writer::write_byte`] so characters supported by
    /// [`Writer::write_byte`] must be used. This includes the entire "Code Page 437" character set,
    /// for now.
    ///
    /// ## Example
    /// ```rust
    /// use crate::drivers::vga_buffer::WRITER;
    /// WRITER.lock().write_string("Hello, World!");
    /// ```
    /// In this example, the string "Hello, World!" is written to the screen. The character is drawn
    /// using the current foreground / background [`Colour`]s.
    ///
    /// ## See also:
    /// * [Code Page 437 (Wikipedia)](https://en.wikipedia.org/wiki/Code_page_437)
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    /// Sets the foreground and background [`Colour`]s of this [`Writer`] instance. The [`Colour`]s
    /// will be used for the next character written.
    ///
    /// ## Example
    /// ```rust
    /// use crate::drivers::vga_buffer::WRITER;
    /// WRITER.lock().set_colour_code(Colour::White, Colour::Red);
    /// ```
    /// In this example, the foreground and background [`Colour`]s are set to white and red respectively.
    /// Any future calls to [`Writer::write_byte`] or [`Writer::write_string`] will use these colours.
    pub fn set_colour_code(&mut self, foreground: Colour, background: Colour) {
        self.colour_code = ColourCode::new(foreground, background);
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            colour_code: self.colour_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// stop one-time initialisation of statics with non-const functions because Rust does
// not know of the VGA memory at `0xb8000` so it can't compute it's value at compile time.
lazy_static! {
    /// This is a global (static) instance of the [`Writer`] struct. It is used to write to the screen.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Definition for Rust print macros to use the VGA screen.

#[macro_export]
/// Prints a message to the VGA buffer. Use this exactly like the `print` macro from the Rust standard
/// library. Internally it calls [`Writer::write_string`] through [`fmt::Write`]'s `write_str`
/// implementation at [`src/drivers/vga_buffer.rs:198`](../src/gtmos/drivers/vga_buffer.rs.html#198-200).
///
/// ## Example
/// ```rust
/// print!("Hello, world!");
/// ```
/// In this example, the string "Hello, world!" is printed to the VGA buffer.
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
/// Prints a message with a new line to the VGA buffer. Use this exactly like the `print` macro from
/// the Rust standard library. Internally it calls [`Writer::write_string`] through [`fmt::Write`]'s `write_str`
/// implementation at [`src/drivers/vga_buffer.rs:198`](../src/gtmos/drivers/vga_buffer.rs.html#198-200).
///
/// ## Example
/// ```rust
/// println!("Hello, world!");
/// ```
/// In this example, the string "Hello, world!" is printed to the VGA buffer and a new line is added.
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

// Tests

#[test_case]
/// Just prints something to the VGA buffer
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
/// Ensure that no panic occurs even if many lines are printed
/// and lines are shifted off the screen
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
/// Verify that the printed lines really appear on the screen
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}
