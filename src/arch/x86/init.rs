use crate::arch::x86::interrupts;
use crate::devices;
use crate::sys::init::KernelInit;
use log::info;
use x86_64::registers::control::{Cr0, Cr0Flags, Cr4, Cr4Flags};
use x86_64::registers::xcontrol::{XCr0, XCr0Flags};

pub struct X86init;
impl KernelInit for X86init {
    unsafe extern "C" fn secondary_cpu_init(cpu: &limine::smp::Cpu) -> ! {
        // Initialize Trap Frame
        unsafe {
            trapframe::init();
        }

        // APIC Initialization
        interrupts::apic::Apic::init_lapic_ap();

        info!("Secondary CPU {} Initialized", cpu.id);
        crate::secondary_main()
    }
    fn stage0() {
        // Detect CPU Features
        let cpuid = raw_cpuid::CpuId::new();
        if let Some(finfo) = cpuid.get_feature_info() {
            if finfo.has_sse()
                || finfo.has_sse2()
                || finfo.has_sse3()
                || finfo.has_ssse3()
                || finfo.has_sse41()
                || finfo.has_sse42()
            {
                let cr0 = Cr0::read() & !Cr0Flags::EMULATE_COPROCESSOR | Cr0Flags::MONITOR_COPROCESSOR;
                unsafe {
                    Cr0::write(cr0);
                }

                let cr4 = Cr4::read() | Cr4Flags::OSFXSR | Cr4Flags::OSXMMEXCPT_ENABLE;
                unsafe {
                    Cr4::write(cr4);
                }
            }

            if finfo.has_xsave() {
                let cr4 = Cr4::read() | Cr4Flags::OSXSAVE;
                unsafe {
                    Cr4::write(cr4);
                }
            }

            if finfo.has_avx() {
                let xcr0 = XCr0::read() | XCr0Flags::AVX | XCr0Flags::SSE;
                unsafe {
                    XCr0::write(xcr0);
                }
            }
        }
    }

    fn stage2() {
        // Load ACPI
        devices::acpi::module_init();

        // IDT Load
        interrupts::module_init();

        // Secondary CPU Initialization
        Self::secondary_init();
    }
}