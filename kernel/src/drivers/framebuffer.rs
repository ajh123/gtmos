use core::cell::RefCell;

use spin::{Mutex, MutexGuard};

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    /// blue channel
    pub b: u8,
    /// green channel
    pub g: u8,
    /// red channel
    pub r: u8,
    /// this byte is 0 in BGR mode, in BGRA, it is alpha.
    pub channel: u8,
}

/// Represents a framebuffer memory region and other metadata used to control
/// different functions of framebuffer.
pub struct FramebufferMemory<'a> {
    /// contains a reference to framebuffer slice
    pub buffer: &'a mut[u8],
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
}


pub struct FramebufferIndex {
    pub x: usize,
    pub y: usize,
}

/// Set of control functions used for writing pixels to frame buffer
pub struct Framebuffer;

impl Framebuffer {
    #[inline]
    pub fn index_in_bounds(fb: &FramebufferMemory, index: &FramebufferIndex) -> bool {
        index.x < fb.width && index.y < fb.height
    }

    #[inline]
    fn index_to_offset(fb: &FramebufferMemory, index: FramebufferIndex) -> Option<usize> {
        if Framebuffer::index_in_bounds(&fb, &index) {
            Some((index.y * fb.width + index.x) * fb.bytes_per_pixel)
        } else {
            None
        }
    }

    #[inline]
    pub fn set_pixel(fb: &mut FramebufferMemory, pixel: Pixel, index: FramebufferIndex) {
        if let Some(offset) = Framebuffer::index_to_offset(&fb, index) {
            fb.buffer[offset] = pixel.b;
            fb.buffer[offset + 1] = pixel.g;
            fb.buffer[offset + 2] = pixel.r;
            fb.buffer[offset + 3] = pixel.channel;
        }
    }

    #[inline]
    pub fn get_pixel(fb: FramebufferMemory, index: FramebufferIndex) -> Option<Pixel> {
        if let Some(offset) = Framebuffer::index_to_offset(&fb, index) {
            return Some(Pixel {
                b: fb.buffer[offset],
                g: fb.buffer[offset + 1],
                r: fb.buffer[offset + 2],
                channel: fb.buffer[offset + 4],
            });
        }

        None
    }

    pub fn fill(fb: &mut MutexGuard<FramebufferMemory>, pixel: Pixel) {
        let bps = fb.bytes_per_pixel;
        Framebuffer::fill_region(fb.buffer, pixel, bps);
    }

    pub fn fill_region(fb_region_slice: &mut [u8], pixel: Pixel, bps: usize) {
        if fb_region_slice.len() % bps != 0 {
            return;
        }

        let mut offset = 0;
        while offset < fb_region_slice.len() {
            fb_region_slice[offset] = pixel.b;
            fb_region_slice[offset + 1] = pixel.g;
            fb_region_slice[offset + 2] = pixel.r;
            fb_region_slice[offset + 3] = pixel.channel;
            offset += bps;
        }
    }
}
