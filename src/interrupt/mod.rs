use lazy_static::lazy_static;

use crate::arch;
use crate::common::structs::cell::Cell;

lazy_static!(
    pub static ref INTERRUPT_MANAGER: InterruptManager = InterruptManager::new();
);

pub struct InterruptManager {
    isrs: Cell<[Option<unsafe fn()>; arch::INTERRUPT_VECTOR_NUM]>,
}

impl InterruptManager {
    pub fn new() -> Self {
        Self {
            isrs: Cell::new([None; 256])
        }
    }

    pub fn register_isr(&self, isr: unsafe fn(), index: usize) -> bool {
        let isrs = self.isrs.get_mut();
        if index > isrs.len() || isrs[index].is_some() {
            return false;
        }
        isrs[index] = Some(isr);
        true
    }

    pub fn unregister_isr(&self, index: usize) {
        self.isrs.get_mut()[index] = None;
    }

    pub unsafe fn handle_interrupt(&self, index: usize) {
        if let Some(isr) = self.isrs.get_mut()[index] {
            isr();
        } else {
            panic!("No ISR registered for interrupt: {}, missing drivers?", index);
        }
    }
}