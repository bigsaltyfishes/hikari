use crate::boot::BOOTINFO;
use crate::common::structs::mem::address::{PhysicalAddress, VirtualAddress};

/// Memory Hardware Abstraction Layer
pub trait AddressSpaceHAL {
    /// Convert a physical address to a virtual address.
    fn phys_to_virt(phys: PhysicalAddress) -> VirtualAddress {
        phys + BOOTINFO.physics_mem_offset
    }

    /// Convert a virtual address to a physical address.
    fn virt_to_phys(virt: VirtualAddress) -> PhysicalAddress {
        virt - BOOTINFO.physics_mem_offset
    }

    /// Read data from a physical address to a buffer.
    fn read_phys(phys: PhysicalAddress, buffer: &mut [u8]) {
        let virt = Self::phys_to_virt(phys) as *const u8;
        unsafe {
            core::ptr::copy::<u8>(virt, buffer.as_mut_ptr(), buffer.len());
        }
    }

    /// Write data from a buffer to a physical address.
    fn write_phys(phys: PhysicalAddress, buffer: &[u8]) {
        let virt = Self::phys_to_virt(phys) as *mut u8;
        unsafe {
            core::ptr::copy::<u8>(buffer.as_ptr(), virt, buffer.len());
        }
    }

    /// Zero out a physical address.
    fn zero_phys(phys: PhysicalAddress, len: usize) {
        let virt = Self::phys_to_virt(phys) as *mut u8;
        unsafe {
            core::ptr::write_bytes::<u8>(virt, 0, len);
        }
    }

    /// Copy data from one physical address to another.
    fn copy_phys(src: PhysicalAddress, dst: PhysicalAddress, len: usize) {
        let src_virt = Self::phys_to_virt(src) as *const u8;
        let dst_virt = Self::phys_to_virt(dst) as *mut u8;
        unsafe {
            core::ptr::copy::<u8>(src_virt, dst_virt, len);
        }
    }

    /// Flush the physical frame
    fn flush_frame(phys: PhysicalAddress);
}