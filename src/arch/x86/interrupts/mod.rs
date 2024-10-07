use alloc::sync::Arc;
use crate::interrupt;

mod trap;
pub mod apic;

pub fn module_init() {
    unsafe {
        trapframe::init();
    }

    // Initialize APIC
    apic::Apic::init_lapic_bsp();
    let irq_ctl = Arc::new(apic::Apic::new());
    interrupt::set_interrupt_controller(irq_ctl);
}