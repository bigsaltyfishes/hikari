use core::arch::asm;

use crate::{drivers, kmain};
use crate::arch::x86::{gdt, interrupts, paging};
use crate::common::debug::console;
use crate::common::debug::logger;
use crate::drivers::early_load_drivers;
use crate::memory;

#[no_mangle]
extern "C" fn _start() -> ! {
    // Initialize logger
    logger::module_init();

    // Initialize SSE
    init_sse();

    // Load Early Drivers
    early_load_drivers();

    // Load Memory Module
    unsafe {
        paging::Mapper::init();
    }
    memory::module_init();

    // IDT/GDT Load
    gdt::module_init();
    interrupts::module_init();

    // Create log buffer
    logger::create_buffer();

    // Loader Late Drivers
    let func = drivers::graphics::framebuffer::module_init().init;
    func();

    // Add framebuffer as log output
    console::set_global_console();

    unsafe { kmain() }
}

fn init_sse() {
    unsafe {
        asm!(
            "mov rax, cr0",
            "and ax, 0xfffb",
            "or ax, 0x2",
            "mov cr0, rax",
            "mov rax, cr4",
            "or ax, 0x600",
            "mov cr4, rax",
            out("rax") _,
        )
    }
}