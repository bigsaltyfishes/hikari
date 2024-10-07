use core::arch::x86_64::{__cpuid, _mm_clflush, _mm_mfence};
use x86_64::structures::paging::{OffsetPageTable, PageTable, Translate};
use crate::mem::address::PhysicalAddress;
use crate::mem::defs::PAGE_BITS;
use crate::hal::memory::utils::MemoryHAL;

pub mod vm;
pub mod table;

pub struct MemoryHALImpl;
impl MemoryHALImpl {
    pub fn cacheline_size() -> usize {
        let leaf = unsafe { __cpuid(1).ebx };
        (((leaf >> 8) & 0xff) << 3) as usize
    }
}

impl MemoryHAL for MemoryHALImpl {
    fn flush_frame(phys: PhysicalAddress) {
        unsafe {
            for paddr in (phys..phys + (1 << PAGE_BITS)).step_by(Self::cacheline_size()) {
                _mm_clflush(Self::phys_to_virt(paddr) as *const u8);
            }
            _mm_mfence();
        }
    }
}