use crate::abstracts::memory::address::AddressSpaceHAL;
use crate::common::structs::mem::address::PhysicalAddress;
use crate::common::structs::mem::misc::PAGE_BITS;
use core::arch::x86_64::{__cpuid, _mm_clflush, _mm_mfence};
use x86_64::structures::paging::Translate;

pub mod vm;
pub mod table;

pub struct AddressSpaceHALImpl;
impl AddressSpaceHALImpl {
    pub fn cacheline_size() -> usize {
        let leaf = unsafe { __cpuid(1).ebx };
        (((leaf >> 8) & 0xff) << 3) as usize
    }
}

impl AddressSpaceHAL for AddressSpaceHALImpl {
    fn flush_frame(phys: PhysicalAddress) {
        unsafe {
            for paddr in (phys..phys + (1 << PAGE_BITS)).step_by(Self::cacheline_size()) {
                _mm_clflush(Self::phys_to_virt(paddr) as *const u8);
            }
            _mm_mfence();
        }
    }
}