use crate::common::structs::interrupt::manager::IrqResult;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::cell::Cell;

pub type IrqHandler = Box<dyn Fn(usize) + Send + Sync>;

#[derive(Debug)]
pub enum IrqTriggerMode {
    Edge,
    Level,
}

#[derive(Debug)]
pub enum IrqPolarity {
    ActiveHigh,
    ActiveLow,
}

pub trait InterruptController {
    fn wait_for_interrupt(&self) {
        core::hint::spin_loop();
    }
    fn is_valid_irq(&self, vector: usize) -> bool;
    fn configure(&self, vector: usize, tm: IrqTriggerMode, pol: IrqPolarity) -> IrqResult {
        unimplemented!();
    }
    fn enable_interrupt(&self) -> IrqResult;
    fn disable_interrupt(&self) -> IrqResult;
    fn is_interrupt_enabled(&self) -> bool;
    fn mask_irq(&self, vector: usize) -> IrqResult;
    fn unmask_irq(&self, vector: usize) -> IrqResult;
    fn register_irq_handler(&self, vector: usize, handler: IrqHandler) -> IrqResult;
    fn unregister_irq_handler(&self, vector: usize) -> IrqResult;
    fn handle_irq(&self, vector: usize) -> IrqResult;
}

pub struct GlobalInterruptController {
    controller: Cell<Option<Arc<dyn InterruptController + Send + Sync>>>,
}