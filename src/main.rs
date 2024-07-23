#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(strict_provenance)]
#![feature(isqrt)]
#![no_std]
#![no_main]
extern crate alloc;

use core::arch::asm;

use crate::interrupt::INTERRUPT_MANAGER;

mod arch;
mod kinfo;
mod boot;
mod memory;
mod drivers;
mod common;
mod panic;
mod interrupt;

unsafe fn kmain() -> ! {
    INTERRUPT_MANAGER.register_isr(test, 128);
    asm!("int 128");
    panic!("内核功能尚未完备，暂时无法继续运行。");
}

unsafe fn test() {
    info!("Hello, World!");
}

fn hcf() -> ! {
    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}