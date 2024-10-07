#![no_std]
#![no_main]
extern crate alloc;

use crate::boot::BOOTINFO;
use crate::common::debug::console::CONSOLE_INSTANCE;
use abstracts::memory::table::GenericPageTable;
use alloc::boxed::Box;
use common::structs::interrupt;
use core::arch::asm;
use core::fmt::Write;
use log::{debug, info};
use raw_cpuid::CpuId;
use sys::mem::paging::PageTable;

mod arch;
mod kinfo;
mod boot;
mod devices;
mod common;
mod panic;
mod sys;
mod abstracts;

unsafe fn kmain() -> ! {
    let interrupt_controller = sys::interrupt::get_ic();
    interrupt_controller.as_ref().register_irq_handler(0, Box::new(|_| {
        test();
    })).unwrap();
    asm!("int 32");
    panic!("内核功能尚未完备，暂时无法继续运行。");
}

unsafe fn secondary_main() -> ! {
    // TODO: Wait for kernel exit
    asm!("int 32");
    loop {
        asm!("hlt");
    }
}

unsafe fn test() {
    let cpuid = CpuId::new().get_feature_info().unwrap().initial_local_apic_id();
    debug!("Hello World from CPU {}", cpuid);
}