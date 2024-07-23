use core::arch::asm;

use crate::registers;

registers!(
    registers => (
        RAX(0),
        RDX(1),
        RCX(2),
        RBX(3),
        RSI(4),
        RDI(5),
        RBP(6),
        RSP(7),
        R8(8),
        R9(9),
        R10(10),
        R11(11),
        R12(12),
        R13(13),
        R14(14),
        R15(15),
        RIP(16),
        XMM0(17),
        XMM1(18),
        XMM2(19),
        XMM3(20),
        XMM4(21),
        XMM5(22),
        XMM6(23),
        XMM7(24),
        XMM8(25),
        XMM9(26),
        XMM10(27),
        XMM11(28),
        XMM12(29),
        XMM13(30),
        XMM14(31),
        XMM15(32),
        RFLAGS(255),
        CS(256),
        FS(257),
        GS(258),
    ),
    implement => {
        fn get(&self) -> Option<usize> {
            let mut value: usize = 0;
            unsafe {
                match self {
                    Registers::RIP => asm!("lea {}, [rip]", out(reg) value, options(nostack, nomem, preserves_flags)),
                    Registers::RSP => asm!("mov {}, rsp", out(reg) value),
                    Registers::RAX => asm!("mov {}, rax", out(reg) value),
                    Registers::RDX => asm!("mov {}, rdx", out(reg) value),
                    Registers::RCX => asm!("mov {}, rcx", out(reg) value),
                    Registers::RBX => asm!("mov {}, rbx", out(reg) value),
                    Registers::RSI => asm!("mov {}, rsi", out(reg) value),
                    Registers::RDI => asm!("mov {}, rdi", out(reg) value),
                    Registers::RBP => asm!("mov {}, rbp", out(reg) value),
                    Registers::R8 => asm!("mov {}, r8", out(reg) value),
                    Registers::R9 => asm!("mov {}, r9", out(reg) value),
                    Registers::R10 => asm!("mov {}, r10", out(reg) value),
                    Registers::R11 => asm!("mov {}, r11", out(reg) value),
                    Registers::R12 => asm!("mov {}, r12", out(reg) value),
                    Registers::R13 => asm!("mov {}, r13", out(reg) value),
                    Registers::R14 => asm!("mov {}, r14", out(reg) value),
                    Registers::R15 => asm!("mov {}, r15", out(reg) value),
                    Registers::RFLAGS => asm!("pushfq; pop {}", out(reg) value),
                    Registers::CS => asm!("mov {}, cs", out(reg) value),
                    Registers::FS => asm!("mov {}, fs", out(reg) value),
                    Registers::GS => asm!("mov {}, gs", out(reg) value),
                    _ => {
                        target_feature_required_block!("sse", {
                            match self {
                                Registers::XMM0 => asm!("movaps {}, xmm0", out(xmm_reg) value),
                                Registers::XMM1 => asm!("movaps {}, xmm1", out(xmm_reg) value),
                                Registers::XMM2 => asm!("movaps {}, xmm2", out(xmm_reg) value),
                                Registers::XMM3 => asm!("movaps {}, xmm3", out(xmm_reg) value),
                                Registers::XMM4 => asm!("movaps {}, xmm4", out(xmm_reg) value),
                                Registers::XMM5 => asm!("movaps {}, xmm5", out(xmm_reg) value),
                                Registers::XMM6 => asm!("movaps {}, xmm6", out(xmm_reg) value),
                                Registers::XMM7 => asm!("movaps {}, xmm7", out(xmm_reg) value),
                                Registers::XMM8 => asm!("movaps {}, xmm8", out(xmm_reg) value),
                                Registers::XMM9 => asm!("movaps {}, xmm9", out(xmm_reg) value),
                                Registers::XMM10 => asm!("movaps {}, xmm10", out(xmm_reg) value),
                                Registers::XMM11 => asm!("movaps {}, xmm11", out(xmm_reg) value),
                                Registers::XMM12 => asm!("movaps {}, xmm12", out(xmm_reg) value),
                                Registers::XMM13 => asm!("movaps {}, xmm13", out(xmm_reg) value),
                                Registers::XMM14 => asm!("movaps {}, xmm14", out(xmm_reg) value),
                                Registers::XMM15 => asm!("movaps {}, xmm15", out(xmm_reg) value),
                                _ => return None,
                            }
                        }, {
                            return None;
                        });
                    }
                }
                Some(value)
            }
        }

        unsafe fn set(&self, value: usize) -> Result<(), ()> {
            match self {
                Registers::RSP => asm!("mov rsp, {}", in(reg) value),
                Registers::RAX => asm!("mov rax, {}", in(reg) value),
                Registers::RDX => asm!("mov rdx, {}", in(reg) value),
                Registers::RCX => asm!("mov rcx, {}", in(reg) value),
                Registers::RBX => asm!("mov rbx, {}", in(reg) value),
                Registers::RSI => asm!("mov rsi, {}", in(reg) value),
                Registers::RDI => asm!("mov rdi, {}", in(reg) value),
                Registers::RBP => asm!("mov rbp, {}", in(reg) value),
                Registers::R8 => asm!("mov r8, {}", in(reg) value),
                Registers::R9 => asm!("mov r9, {}", in(reg) value),
                Registers::R10 => asm!("mov r10, {}", in(reg) value),
                Registers::R11 => asm!("mov r11, {}", in(reg) value),
                Registers::R12 => asm!("mov r12, {}", in(reg) value),
                Registers::R13 => asm!("mov r13, {}", in(reg) value),
                Registers::R14 => asm!("mov r14, {}", in(reg) value),
                Registers::R15 => asm!("mov r15, {}", in(reg) value),
                _ => return Err(())
            }
            Ok(())
        }
    }
);