use crate::common::debug::unwind::trace::Tracer;
use x86::current::registers::rbp;

pub use X86StackTrace as StackTracer;

pub struct X86StackTrace {
    rbp: *const usize,
}

impl Tracer for X86StackTrace {
    fn new() -> Self {
        let rbp: *const usize = rbp() as *const _;
        Self { rbp }
    }

    fn next(&mut self) -> Option<usize> {
        if self.rbp.is_null() {
            return None;
        }
        let ra = unsafe { *self.rbp.offset(1) };
        self.rbp = unsafe { *self.rbp as *const usize };
        if ra != 0 { Some(ra - 1) } else { None }
    }
}