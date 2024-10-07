use crate::abstracts::trap::TrapReason;
use crate::common::structs::interrupt;
use crate::common::structs::mem::misc::MMUFlags;
use crate::sys;
use log::{error, info, trace};
use raw_cpuid::CpuId;
use trapframe::TrapFrame;
use x86::irq::{ALIGNMENT_CHECK_VECTOR, BREAKPOINT_VECTOR, DEBUG_VECTOR, INVALID_OPCODE_VECTOR, PAGE_FAULT_VECTOR};

impl TrapReason {
    pub fn from(trap_num: usize, error_code: usize) -> Self {
        use x86::irq::*;
        const X86_INT_BASE: u8 = 0x20;
        const X86_INT_MAX: u8 = 0xff;

        // See https://github.com/rcore-os/trapframe-rs/blob/25cb5282aca8ceb4f7fc4dcd61e7e73b67d9ae00/src/arch/x86_64/syscall.S#L117
        if trap_num == 0x100 {
            return Self::Syscall;
        }
        match trap_num as u8 {
            DEBUG_VECTOR => Self::HardwareBreakpoint,
            BREAKPOINT_VECTOR => Self::SoftwareBreakpoint,
            INVALID_OPCODE_VECTOR => Self::UndefinedInstruction,
            ALIGNMENT_CHECK_VECTOR => Self::UnalignedAccess,
            PAGE_FAULT_VECTOR => {
                bitflags::bitflags! {
                    struct PageFaultErrorCode: u32 {
                        const PRESENT =     1 << 0;
                        const WRITE =       1 << 1;
                        const USER =        1 << 2;
                        const RESERVED =    1 << 3;
                        const INST =        1 << 4;
                    }
                }
                let fault_vaddr = x86_64::registers::control::Cr2::read().unwrap().as_u64() as _;
                let code = PageFaultErrorCode::from_bits_truncate(error_code as u32);
                let mut flags = MMUFlags::empty();
                if code.contains(PageFaultErrorCode::WRITE) {
                    flags |= MMUFlags::WRITE
                } else {
                    flags |= MMUFlags::READ
                }
                if code.contains(PageFaultErrorCode::USER) {
                    flags |= MMUFlags::USER
                }
                if code.contains(PageFaultErrorCode::INST) {
                    flags |= MMUFlags::EXECUTE
                }
                if code.contains(PageFaultErrorCode::RESERVED) {
                    error!("page table entry has reserved bits set!");
                }
                Self::PageFault(fault_vaddr, flags)
            }
            vec @ X86_INT_BASE..=X86_INT_MAX => Self::Interrupt(vec as usize),
            _ => Self::GernelFault(trap_num),
        }
    }
}

#[no_mangle]
pub extern "C" fn trap_handler(tf: &mut TrapFrame) {
    let cpuid = CpuId::new().get_feature_info().unwrap().initial_local_apic_id();
    trace!(
        "Interrupt: {:#x} @ CPU{}",
        tf.trap_num,
        cpuid
    );

    match TrapReason::from(tf.trap_num, tf.error_code) {
        TrapReason::HardwareBreakpoint | TrapReason::SoftwareBreakpoint => info!("Breakpoint"),
        TrapReason::PageFault(vaddr, flags) => panic!(
            "Page fault at {:#x} with flags {:?} @ CPU{}\n{:#x?}",
            vaddr, flags, cpuid, tf
        ),
        TrapReason::Interrupt(vector) => {
            sys::interrupt::get_ic().handle_irq(vector).unwrap()
        }
        other => panic!("Unhandled trap {:x?} {:#x?}", other, tf),
    }
}