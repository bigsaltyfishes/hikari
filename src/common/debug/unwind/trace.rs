use core::ffi::{c_int, c_void};

use unwinding::abi::{_Unwind_Backtrace, _Unwind_GetGR, _Unwind_GetIP, UnwindContext, UnwindReasonCode};

use crate::{arch, trace};
use crate::arch::ARCHITECTURE_MAX_DWARF_REGS;
use crate::common::debug::symbols::KERNEL_SYMBOLS;
use crate::common::structs::register::Register;
use crate::kinfo::KERNEL_STACK_TRACE_FRAME_NUM;

pub fn stack_trace() {
    #[derive(Default)]
    struct Context {
        ip: [usize; KERNEL_STACK_TRACE_FRAME_NUM],
        regs: [usize; ARCHITECTURE_MAX_DWARF_REGS],
        counter: usize,
    }
    extern "C" fn callback(unwind_ctx: &UnwindContext<'_>, arg: *mut c_void) -> UnwindReasonCode {
        let data = unsafe { &mut *(arg as *mut Context) };
        let ip = _Unwind_GetIP(unwind_ctx);
        if ip == 0 {
            return UnwindReasonCode::NORMAL_STOP;
        }
        if data.counter >= KERNEL_STACK_TRACE_FRAME_NUM {
            return UnwindReasonCode::NORMAL_STOP;
        } else if data.counter == 0 {
            for reg in 0..ARCHITECTURE_MAX_DWARF_REGS {
                data.regs[reg] = _Unwind_GetGR(unwind_ctx, reg as c_int);
            }
        }
        data.ip[data.counter] = ip;
        data.counter += 1;
        UnwindReasonCode::NO_REASON
    }
    let mut data = Context::default();
    _Unwind_Backtrace(callback, &mut data as *mut _ as _);

    trace!("Registers: ");
    let mut print_queue: [Option<(&str, usize)>; 2] = [None; 2];
    for reg in 0..ARCHITECTURE_MAX_DWARF_REGS {
        let value = data.regs[reg];
        let register_name = arch::Registers::try_from(reg).map(|r| r.name()).unwrap();
        if value != 0 {
            if print_queue[0].is_none() {
                print_queue[0] = Some((register_name, value));
            } else if print_queue[1].is_none() {
                print_queue[1] = Some((register_name, value));
            }
            if print_queue[0].is_some() && print_queue[1].is_some() {
                trace!("{}: {:#x} {}: {:#x}", print_queue[0].unwrap().0, print_queue[0].unwrap().1, print_queue[1].unwrap().0, print_queue[1].unwrap().1);
                print_queue[0] = None;
                print_queue[1] = None;
            }
        }
    }
    if print_queue[0].is_some() {
        trace!("{}: {:#x}", print_queue[0].unwrap().0, print_queue[0].unwrap().1);
    }

    trace!("STACK TRACE: ");
    for i in 0..data.counter {
        let pc = data.ip[i] - 1;
        KERNEL_SYMBOLS.find_symbol(pc).map(|(function_name, offset)| {
            trace!("{:4}:<{:#x}> - <{:#} + {:#x}>", i, pc, function_name, offset);
        }).unwrap_or_else(|| {
            trace!("{:4}:<{:#x}> - <? + ?>", i, pc);
        });
    }
}