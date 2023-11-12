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
static mut PLATFORM: Option<RefCell<Option<&'static mut dyn SubSystem>>> = None;

// Function to set the static reference in the main crate
#[allow(mutable_transmutes)]
pub fn set_platform<T: SubSystem>(platform: Platform<T>) {
    unsafe {
        if PLATFORM.is_none() {
            let static_ref: &'static mut dyn SubSystem =
                mem::transmute(&platform.sub_system as &dyn SubSystem);
            PLATFORM = Some(RefCell::new(Some(static_ref)));
        }
    }
}

pub fn get_sub_system() -> Option<&'static mut dyn SubSystem> {
    unsafe {
        let r: Option<&'static mut dyn SubSystem> = PLATFORM
            .as_mut()
            .and_then(|p| {
                // Take out the inner value and replace it with None temporarily
                let inner_ref = p.borrow_mut().take()?;

                // Convert the reference to 'static
                let static_ref: &'static mut (dyn SubSystem + 'static) = mem::transmute(inner_ref);

                // Create a reference with 'static lifetime
                Some(static_ref)
            });
        return r;
    }
}
