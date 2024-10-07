use core::ffi::{c_int, c_void};
use log::trace;
#[cfg(feature = "dwarf-unwind")]
use unwinding::abi::{UnwindContext, UnwindReasonCode, _Unwind_Backtrace, _Unwind_GetGR, _Unwind_GetIP};

use crate::arch;
use crate::arch::ARCHITECTURE_MAX_DWARF_REGS;
use crate::common::debug::symbols::KERNEL_SYMBOLS;
use crate::kinfo::KERNEL_STACK_TRACE_FRAME_NUM;

#[cfg(not(feature = "dwarf-unwind"))]
use crate::arch::hal_impl::trace::StackTracer;

pub trait Tracer {
    /// Create a new stack tracer
    fn new() -> Self;
    /// Get the next frame's instruction pointer
    fn next(&mut self) -> Option<usize>;
}

#[cfg(feature = "dwarf-unwind")]
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

#[cfg(not(feature = "dwarf-unwind"))]
pub fn stack_trace() {
    let mut tracer = StackTracer::new();
    let mut counter = 0;
    trace!("STACK TRACE: ");
    while let Some(ra) = tracer.next() {
        counter += 1;
        KERNEL_SYMBOLS.find_symbol(ra).map(|(function_name, offset)| {
            trace!("{:4}:<{:#x}> - <{:#} + {:#x}>", counter, ra, function_name, offset);
        }).unwrap_or_else(|| {
            trace!("{:4}:<{:#x}> - <? + ?>", counter, ra);
        });
    }
}