use log::info;

use crate::boot::BOOTINFO;
use crate::common::debug::{console, logger};
use crate::devices::efifb;
use crate::devices::uart;
use crate::kinfo;
use crate::sys::mem;
use crate::sys::mem::heap;

/// Kernel Boot Stage
///
/// Stage 0: Early Initialize CPU Features
/// Stage 1: Initialize Base Kernel Modules and Subsystems
/// Stage 2: Architecture Specific Initialization (For example, ACPI/APIC/GDT/IDT Setup)
pub trait KernelInit {
    fn secondary_init() {
        let limine_smp = BOOTINFO.smp_response;
        let bsp_lapic_id = limine_smp.bsp_lapic_id();

        for cpu in limine_smp.cpus() {
            if cpu.id == bsp_lapic_id {
                continue;
            }

            cpu.goto_address.write(Self::secondary_cpu_init);
        }
    }
    unsafe extern "C" fn secondary_cpu_init(cpu: &limine::smp::Cpu) -> ! {
        unsafe {
            trapframe::init();
        }
        info!("Secondary CPU {} Initialized", cpu.id);
        crate::secondary_main()
    }
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
        heap::module_init();
    }
    fn stage2();
}

pub fn kernel_init<T: KernelInit>() {
    T::stage0();
    T::stage1();
    T::stage2();
}