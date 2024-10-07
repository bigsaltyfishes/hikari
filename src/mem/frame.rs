use alloc::vec::Vec;
use crate::hal::memory::utils::MemoryHAL;
use crate::{hal, mem};
use crate::mem::address::PhysicalAddress;
use crate::mem::defs::PAGE_BITS;

/// A 4KB page frame
pub struct PhysicalFrame {
    phys_addr: PhysicalAddress,
    from_allocator: bool,
}

impl PhysicalFrame {
    pub fn new() -> Option<Self> {
        mem::frame_alloc(1, 0).map(|phys_addr| Self {
            phys_addr,
            from_allocator: true
        })
    }

    pub fn new_with_zero() -> Option<Self> {
        Self::new().map(|mut frame| {
            frame.zero();
            frame
        })
    }

    // TODO: Unexpected unwrap here, should be handled properly
    pub fn new_contiguous(frame_count: usize, align_log2: usize) -> Vec<Self> {
        mem::frame_alloc(frame_count, align_log2).map_or(Vec::new(), |phys_addr| {
            (0..frame_count).map(|i| Self {
                phys_addr: phys_addr + (i << PAGE_BITS),
                from_allocator: true
            }).collect()
        })
    }

    pub unsafe fn from_phys(phys_addr: PhysicalAddress) -> Self {
        assert!(mem::address::is_aligned(phys_addr));
        Self {
            phys_addr,
            from_allocator: false
        }
    }

    pub fn phys_addr(&self) -> PhysicalAddress {
        self.phys_addr
    }

    pub fn as_ptr(&self) -> *const u8 {
        hal::memory::utils::implement::phys_to_virt(self.phys_addr) as *const u8
    }

    pub fn as_mut_ptr(&self) -> *mut u8 {
        hal::memory::utils::implement::phys_to_virt(self.phys_addr) as *mut u8
    }

    pub fn zero(&mut self) {
        hal::memory::utils::implement::zero_phys(self.phys_addr, 4096);
    }
}

impl Drop for PhysicalFrame {
    fn drop(&mut self) {
        if self.from_allocator {
            mem::frame_dealloc(self.phys_addr);
        }
    }
}
