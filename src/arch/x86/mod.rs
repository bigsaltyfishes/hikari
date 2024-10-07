mod entry;
mod interrupts;
mod init;
pub mod hal_impl;

#[cfg(not(feature = "dwarf-unwind"))]
pub use hal_impl::trace::X86StackTrace;

pub const ARCHITECTURE_MAX_DWARF_REGS: usize = 16;

