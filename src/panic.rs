use core::arch::asm;
use lazy_static::lazy_static;
use log::error;
use crate::common::debug::unwind;
use crate::common::structs::cell::Cell;
use crate::interrupt;
use crate::boot::BOOTINFO;
use crate::common::debug::symbols::KERNEL_SYMBOLS;
use crate::common::debug::unwind::trace;

lazy_static!(
    static ref PANIC_COUNTER: Cell<usize> = Cell::new(0);
);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    interrupt::get_interrupt_controller().disable_interrupt().unwrap();
    let panic_counter = PANIC_COUNTER.get_mut();
    *panic_counter += 1;

    error!("⚠!!! KERNEL PANIC !!!⚠");
    if let Some(location) = _info.location() {
        error!("Location: {}:{}:{}", location.file(), location.line(), location.column());
    }
    error!("Reason: {}", _info.message());

    if *panic_counter > 1 {
        error!("⚠!!! DOUBLE PANIC !!!⚠");
        error!("This is a double panic, halting the system.");
        hlt()
    }

    trace::stack_trace();
    hlt()
}

fn hlt() -> ! {
    unsafe {
        asm!("hlt");
    }
    loop {}
}