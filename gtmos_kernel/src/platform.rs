use core::{cell::RefCell, mem};
use crate::console::Console;
pub trait SubSystem {
    fn initialise(&self);
    fn halt(&self);
    fn write(&mut self, dest: &str, data: &str);
    // fn read(&self, dest: &str) -> &str;
    fn set_console(&mut self, console: Option<Console<'static>>);
    fn get_console(&mut self) -> Option<&mut Console<'static>>;
}

pub struct Platform<T: SubSystem> {
    pub sub_system: T
}

impl<T: SubSystem> Platform<T> {
    pub fn new(sub_system: T) -> Self {
        Platform { sub_system }
    }

    pub fn halt(&self) -> ! {
        self.sub_system.halt();
        loop {}
    }
}

// Global static reference to the platform
static mut PLATFORM: Option<RefCell<Option<&'static dyn SubSystem>>> = None;

// Function to set the static reference in the main crate
pub fn set_platform<T: SubSystem>(platform: Platform<T>) {
    unsafe {
        if PLATFORM.is_none() {
            let static_ref: &'static dyn SubSystem =
                mem::transmute(&platform.sub_system as &dyn SubSystem);
            PLATFORM = Some(RefCell::new(Some(static_ref)));
        }
    }
}

#[allow(mutable_transmutes)]
pub fn get_sub_system() -> Option<&'static mut dyn SubSystem> {
    unsafe {
        PLATFORM
            .as_ref()?
            .borrow_mut()
            .take()
            .map(|s| mem::transmute(s))
    }
}
