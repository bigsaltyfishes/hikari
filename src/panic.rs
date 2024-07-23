use core::arch::asm;

use lazy_static::lazy_static;

use crate::{error, hcf};
use crate::common::debug::unwind;
use crate::common::structs::cell::Cell;

lazy_static!(
    static ref PANIC_COUNTER: Cell<usize> = Cell::new(0);
);

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let panic_counter = PANIC_COUNTER.get_mut();
    *panic_counter += 1;
    unsafe {
        asm!("cli");
    }

    error!("⚠!!! KERNEL PANIC !!!⚠");
    if let Some(location) = _info.location() {
        error!("Location: {}:{}:{}", location.file(), location.line(), location.column());
    }
    error!("Reason: {}", _info.message());

    if *panic_counter > 1 {
        error!("⚠!!! DOUBLE PANIC !!!⚠");
        error!("This is a double panic, halting the system.");
        hcf();
    }

    unwind::stack_trace();
    hcf();
}