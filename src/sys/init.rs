use crate::common::debug::{console, logger};
use crate::devices::uart;
use crate::{kinfo, mem};
use crate::devices::efifb;

/// Kernel Boot Stage
///
/// Stage 0: Early Initialize CPU Features
/// Stage 1: Initialize Base Kernel Modules and Subsystems
/// Stage 2: Architecture Specific Initialization (For example, ACPI/APIC/GDT/IDT Setup)
pub trait KernelInit {
    fn stage0();
    fn stage1() {
        // Kernel Logger Initialization
        logger::module_init();

        // Load the UART driver
        uart::module_init();

        // Load Graphics Driver
        efifb::module_init();
        console::module_init();

        // Print System Information
        kinfo::print_sys_info();

        // Initialize Memory Subsystem
        mem::module_init();
    }
    fn stage2();
}

pub fn kernel_init<T: KernelInit>() {
    T::stage0();
    T::stage1();
    T::stage2();
}