use crate::mem::defs::PAGE_BITS;
pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub const fn align_down(addr: usize) -> usize {
    addr & !((1 << PAGE_BITS) - 1)
}

pub const fn align_up(addr: usize) -> usize {
    align_down(addr + (1 << PAGE_BITS) - 1)
}

pub const fn page_offset(addr: usize) -> usize {
    addr & ((1 << PAGE_BITS) - 1)
}

pub const fn page_count(size: usize) -> usize {
    align_up(size) / (1 << PAGE_BITS)
}

pub fn is_aligned(addr: usize) -> bool {
    page_offset(addr) == 0
}