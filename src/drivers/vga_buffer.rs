use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// suppress warning for each unused variant
#[allow(dead_code)]
// enable copy semantics for the type and make it printable and comparable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// store each member as 8 bits. 4 bits would be more efficient but Rust 
// does not have a `u4`.
#[repr(u8)]
/// Represents each VGA colour
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
/// ColourCode represents a combination of foreground and background Colours.
struct ColourCode(u8);

impl ColourCode {
    fn new(foreground: Colour, background: Colour) -> ColourCode {
        ColourCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// make sure this struct is laid out the same as a C struct. 
#[repr(C)]
/// Represents a character on the screen with a ColourCode.
struct ScreenChar {
    ascii_character: u8,
    colour_code: ColourCode,
}

// 25 rows
const BUFFER_HEIGHT: usize = 25;
// 80 columns
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
/// Buffer is a 2D array of ScreenChars, it represents the screen.
struct Buffer {
    // this must be Volatile because rust does not know this is memory mapped IO
    // for VGA and rust will try to optimise it because it does not know the memory
    // will be read by the graphics hardware.
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

/// Writer is a wrapper around Buffer it keeps track of the current colour and 
/// the last row's column position.
pub struct Writer {
    column_position: usize,
    colour_code: ColourCode,
    // use static to make sure this value exists forever.
    buffer: &'static mut Buffer,
}

impl Writer {
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
    /// Global Writer
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Definition for Rust print macros to use the VGA screen.

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
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
