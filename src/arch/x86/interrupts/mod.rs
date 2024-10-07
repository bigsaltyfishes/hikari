use crate::sys;
use alloc::sync::Arc;

mod trap;
pub mod apic;

pub fn module_init() {
    unsafe {
        trapframe::init();
    }

    // Initialize APIC
    apic::Apic::init_lapic_bsp();
    let irq_ctl = Arc::new(apic::Apic::new());
    sys::interrupt::set_ic(irq_ctl);
}