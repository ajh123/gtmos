// use core::fmt::Write;
use core::str;

use crate::drivers::framebuffer::Pixel;
use crate::graphics::GraphicsAPI;

pub struct Console<'a> {
    graphics_api: &'a mut GraphicsAPI<'a>,
    cursor_x: usize,
    cursor_y: usize,
    font_size: usize,
}

impl<'a> Console<'a> {
    pub fn new(graphics_api: &'a mut GraphicsAPI<'a>, font_size: usize) -> Self {
        Console {
            graphics_api,
            cursor_x: 0,
            cursor_y: 0,
            font_size,
        }
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.cursor_x = x;
        self.cursor_y = y;
    }

    pub fn write_str(&mut self, s: &str, text_color: Pixel, background_color: Pixel) {
        for c in s.chars() {
            if c == '\n' {
                // Handle newline character
                self.cursor_x = 0;
                self.cursor_y += self.font_size * 8;
                continue;
            }

            self.graphics_api.draw_char(
                self.cursor_x,
                self.cursor_y,
                c,
                text_color,
                background_color,
                self.font_size,
            );

            self.cursor_x += self.font_size * 8;

            // Check if text exceeds the width of the screen
            if self.cursor_x + self.font_size * 8 > self.graphics_api.get_width() {
                self.cursor_x = 0;
                self.cursor_y += self.font_size * 8;
            }

            // Check if text exceeds the height of the screen, and scroll if needed
            if self.cursor_y + self.font_size * 8 > self.graphics_api.get_height() {
                self.scroll_up(self.font_size * 8);
                self.cursor_y -= self.font_size * 8;
            }
        }
    }

    fn scroll_up(&mut self, lines: usize) {
        let fb_height = self.graphics_api.get_height();
        let fb_width = self.graphics_api.get_width();

        // Copy the entire framebuffer content up by 'lines' pixels
        self.graphics_api.copy_rect(0, lines, fb_width, fb_height - lines, 0, 0, Pixel { r: 0, g: 0, b: 0 });

        // Clear the newly created empty space at the bottom
        self.graphics_api.draw_filled_rectangle(0, fb_height - lines, fb_width, lines, Pixel { r: 0, g: 0, b: 0 });
    }
}
