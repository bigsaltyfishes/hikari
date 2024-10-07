use crate::abstracts::interrupt::controller::InterruptController;
use alloc::sync::Arc;

static mut IRQ: Option<Arc<dyn InterruptController>> = None;

pub fn set_ic(irq: Arc<dyn InterruptController>) {
    unsafe {
        IRQ = Some(irq);
    }
}

pub fn get_ic() -> Arc<dyn InterruptController> {
    unsafe {
        IRQ.as_ref().expect("Interrupt controller not set").clone()
    }
}