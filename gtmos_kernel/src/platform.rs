use core::{cell::RefCell, mem};

pub trait CPU {
    fn initialise(&self);
    fn halt(&self);
    fn write(&self, dest: &str, data: &str);
    // fn read(&self, dest: &str) -> &str;
}

pub struct Platform<T: CPU> {
    pub cpu: T,
}

impl<T: CPU> Platform<T> {
    pub fn new(cpu: T) -> Self {
        Platform { cpu }
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
