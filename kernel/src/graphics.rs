// graphics.rs

use core::cell::RefCell;

use crate::drivers::framebuffer::{Framebuffer, FramebufferIndex, Pixel, FramebufferMemory};

/// A simple graphics API for plotting pixels and drawing rectangles on the framebuffer.

pub struct GraphicsAPI<'a> {
    framebuffer: RefCell<FramebufferMemory<'a>>,
}

impl<'a> GraphicsAPI<'a> {
    pub fn new(framebuffer: RefCell<FramebufferMemory<'a>>) -> Self {
        GraphicsAPI { framebuffer }
    }

    /// Plot a pixel at the specified location (x, y) with the given colour.
    pub fn plot_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        // let fb = *self.framebuffer.get_mut();

        // // let fb_ptr = self.framebuffer.as_ptr();
        // // let fb = unsafe {&mut *fb_ptr };

        // let index = FramebufferIndex { x, y };
        // Framebuffer::set_pixel(fb, pixel, index); // fb needs to be `&mut FramebufferMemory<'_>`
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

    /// Fill the entire framebuffer with the given pixel colour.
    pub fn fill_framebuffer(&mut self, pixel: Pixel) {
        let width = self.framebuffer.borrow().width;
        let height = self.framebuffer.borrow().height;
        self.draw_filled_rectangle(0, 0, width, height, pixel);
    }

    pub fn get_width(&self) -> usize {
        return self.framebuffer.borrow().width;
    }

    pub fn get_height(&self) -> usize {
        return self.framebuffer.borrow().height
    }
}
