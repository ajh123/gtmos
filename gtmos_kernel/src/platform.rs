use core::{cell::RefCell, mem};
use crate::console::Console;

pub trait CPU {
    fn initialise(&self);
    fn halt(&self);
    fn write(&self, dest: &str, data: &str);
    // fn read(&self, dest: &str) -> &str;
}

pub struct Platform<T: CPU> {
    pub cpu: T,
    pub console: Option<Console<'static>>,
}

impl<T: CPU> Platform<T> {
    pub fn new(cpu: T) -> Self {
        Platform { cpu, console: None }
    }

    pub fn set_console(&mut self, console: Console<'static>) {
        self.console = Some(console);
    }

    pub fn initialise(&self) {
        self.cpu.initialise();
    }

    pub fn halt(&self) -> ! {
        self.cpu.halt();
        loop {}
    }
}

// Global static reference to the platform
static mut PLATFORM: Option<RefCell<Option<&'static dyn CPU>>> = None;

// Function to set the static reference in the main crate
pub fn set_platform<T: CPU>(platform: Platform<T>) {
    unsafe {
        if PLATFORM.is_none() {
            let static_ref: &'static dyn CPU =
                mem::transmute(&platform.cpu as &dyn CPU);
            PLATFORM = Some(RefCell::new(Some(static_ref)));
            platform.initialise();
        }
    }
}

// Function to get a reference to the platform's cpu
pub fn get_cpu() -> Option<&'static dyn CPU> {
    unsafe {
        PLATFORM
            .as_ref()
            .and_then(|p| p.borrow().as_ref().cloned())
    }
}

// Function to get a reference to the platform itself
pub fn get_platform<T: CPU + 'static>() -> Option<&'static mut Platform<T>> {
    unsafe {
        PLATFORM
            .as_ref()
            .and_then(|p| {
                let mut mut_ref = p.borrow_mut();
                mut_ref.as_mut().map(|platform| platform as *mut _ as *mut Platform<T>)
            })
            .map(|platform_ptr| &mut *platform_ptr)
    }
}
