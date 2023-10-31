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
pub struct FramebufferMemory {
    /// contains a reference to framebuffer slice
    pub buffer: &'static mut [u8],
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
}

pub struct FramebufferIndex {
    pub x: usize,
    pub y: usize,
}

impl FramebufferMemory {
    /// creates a new frame buffer memory region over the framebuffer area
    /// provided by the bootloader.
    pub fn new(boot_info: &'static mut bootloader_api::BootInfo) -> Option<Self> {
        if let Some(framebuffer_info) = boot_info.framebuffer.as_mut() {
            Some(FramebufferMemory {
                width: framebuffer_info.info().width,
                height: framebuffer_info.info().height,
                bytes_per_pixel: framebuffer_info.info().bytes_per_pixel,
                buffer: framebuffer_info.buffer_mut(),
            })
        } else {
            // log::error!(
            //     "Could not initialize framebuffer,
            //      because the bootloader did not provide framebuffer info."
            // );
            return None;
        }
    }
}

/// LockedFramebuffer represents a framebuffer memory region with mutex.
pub type LockedFramebuffer = Mutex<FramebufferMemory>;

/// initializes the framebuffer
pub(crate) fn init_framebuffer(boot_info: &'static mut bootloader_api::BootInfo) -> Option<Mutex<FramebufferMemory>> {
    let fb_opt = FramebufferMemory::new(boot_info);
    if fb_opt.is_none() {
        return None;
    }

    Some(Mutex::new(fb_opt.unwrap()))
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
    pub fn get_pixel(fb: &FramebufferMemory, index: FramebufferIndex) -> Option<Pixel> {
        if let Some(offset) = Framebuffer::index_to_offset(fb, index) {
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
