use crate::boot::BOOTINFO;
use crate::common::debug::symbols::KERNEL_SYMBOLS;
use crate::common::debug::unwind;
use crate::common::debug::unwind::trace;
use crate::common::structs::interrupt;
use crate::sys;
use core::arch::asm;
use lazy_static::lazy_static;
use log::error;

static mut PANIC_COUNTER: u32 = 0;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    sys::interrupt::get_ic().disable_interrupt().unwrap();
    let panic_counter = unsafe { &mut PANIC_COUNTER };
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