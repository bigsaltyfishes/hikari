use alloc::boxed::Box;
use alloc::sync::Arc;
use core::cell::Cell;
use spin::Mutex;
use crate::interrupt::controller::InterruptController;

pub mod manager;
pub mod controller;

static mut IRQ: Option<Arc<dyn InterruptController>> = None;

pub fn set_interrupt_controller(irq: Arc<dyn InterruptController>) {
    unsafe {
        IRQ = Some(irq);
    }
}

pub fn get_interrupt_controller() -> Arc<dyn InterruptController> {
    unsafe {
        IRQ.as_ref().expect("Interrupt controller not set").clone()
    }
}