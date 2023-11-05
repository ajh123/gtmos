// graphics.rs

use core::cell::RefCell;

use crate::drivers::framebuffer::{Framebuffer, FramebufferIndex, Pixel, FramebufferMemory};
use font8x8::{BASIC_FONTS, UnicodeFonts};

/// A simple graphics API for plotting pixels and drawing rectangles on the framebuffer.

pub struct GraphicsAPI<'a> {
    framebuffer: RefCell<FramebufferMemory<'a>>,
}

impl<'a> GraphicsAPI<'a> {
    pub fn new(framebuffer: RefCell<FramebufferMemory<'a>>) -> Self {
        GraphicsAPI {
            framebuffer: framebuffer,
        }
    }

    /// Plot a pixel at the specified location (x, y) with the given colour.
    pub fn plot_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        let mut fb = self.framebuffer.borrow_mut();
        let index = FramebufferIndex { x, y };
        Framebuffer::set_pixel(&mut fb, pixel, index);
    }

    /// Draw a filled rectangle at the specified location with the given colour.
    pub fn draw_filled_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, pixel: Pixel) {
        for i in y..(y + height) {
            for j in x..(x + width) {
                self.plot_pixel(j, i, pixel);
            }
        }
    }

    /// Draw a line from (x1, y1) to (x2, y2) with the given colour.
    pub fn draw_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, pixel: Pixel) {
        let dx = (x2 as isize - x1 as isize).abs();
        let dy = (y2 as isize - y1 as isize).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1 as isize;
        let mut y = y1 as isize;

        while x != x2 as isize || y != y2 as isize {
            self.plot_pixel(x as usize, y as usize, pixel);

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }


    /// Copy a rectangular region from the source location to the destination location.
    pub fn copy_rect(&mut self, src_x: usize, src_y: usize, width: usize, height: usize, dest_x: usize, dest_y: usize, fill_colour: Pixel) {
        let mut fb = self.framebuffer.borrow_mut();

        for y in 0..height {
            for x in 0..width {
                let src_index = FramebufferIndex { x: src_x + x, y: src_y + y };
                let dest_index = FramebufferIndex { x: dest_x + x, y: dest_y + y };

                // Get the colour at the source location and copy it to the destination
                let src_colour = Framebuffer::get_pixel(&mut fb, src_index).unwrap_or(fill_colour);
                Framebuffer::set_pixel(&mut fb, src_colour, dest_index);
            }
        }
    }

    /// Draws a character at a given location with a font size.
    pub fn draw_char(&mut self, x: usize, y: usize, c: char, text: Pixel, background: Pixel, font_size: usize) {
        if let Some(glyph) = BASIC_FONTS.get(c) {
            let glyph_width = 8;
            let glyph_height = 8;

            for gy in 0..glyph_height {
                for gx in 0..glyph_width {
                    let bit = glyph[gy] & (1 << (7 - gx));
                    if bit != 0 {
                        for dx in 0..font_size {
                            for dy in 0..font_size {
                                self.plot_pixel(x + gx * font_size + dx, y + gy * font_size + dy, text);
                            }
                        }
                    } else {
                        for dx in 0..font_size {
                            for dy in 0..font_size {
                                self.plot_pixel(x + gx * font_size + dx, y + gy * font_size + dy, background);
                            }
                        }
                    }
                }
            }
        }
    }

    /// Draws a character at a given location with a font size with a transparent transparent.
    pub fn draw_char_transparent(&mut self, x: usize, y: usize, c: char, text: Pixel, font_size: usize) {
        if let Some(glyph) = BASIC_FONTS.get(c) {
            let glyph_width = 8;
            let glyph_height = 8;

            for gy in 0..glyph_height {
                for gx in 0..glyph_width {
                    let bit = glyph[gy] & (1 << (7 - gx));
                    if bit != 0 {
                        for dx in 0..font_size {
                            for dy in 0..font_size {
                                self.plot_pixel(x + gx * font_size + dx, y + gy * font_size + dy, text);
                            }
                        }
                    }
                }
            }
        }
    }
}
