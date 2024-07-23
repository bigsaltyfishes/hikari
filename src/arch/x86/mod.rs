pub use registers::Registers;

pub const INTERRUPT_VECTOR_NUM: usize = 256;
pub const ARCHITECTURE_MAX_DWARF_REGS: usize = 16;

mod entry;
mod paging;
mod gdt;
mod interrupts;
mod registers;

