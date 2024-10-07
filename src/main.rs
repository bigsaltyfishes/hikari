#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(strict_provenance)]
#![feature(isqrt)]
#![no_std]
#![no_main]
extern crate alloc;

use alloc::boxed::Box;
use core::arch::asm;
use core::fmt::Write;
use log::info;
use crate::boot::BOOTINFO;
use hal::memory::table::GenericPageTable;
use hal::memory::table::implement::PageTable;
use crate::common::debug::console::CONSOLE_INSTANCE;

mod arch;
mod kinfo;
mod boot;
mod mem;
mod devices;
mod common;
mod panic;
mod sys;
mod hal;
mod interrupt;

unsafe fn kmain() -> ! {
    panic!("内核功能尚未完备，暂时无法继续运行。");
}

unsafe fn test() {
    info!("Hello, World!");
    let mut page_table = PageTable::from_active();
    let (addr, flags, size) = page_table.query((BOOTINFO.kernel_address + BOOTINFO.physics_mem_offset) as _).unwrap();
    info!("addr: {:#x}, flags: {:?}, size: {:?}", addr, flags, size);
}