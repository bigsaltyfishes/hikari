use crate::abstracts::memory::address::AddressSpaceHAL;
use crate::abstracts::memory::table::GenericPTE;
use crate::abstracts::memory::vm::VmHAL;
use crate::arch::x86::hal_impl::memory::table::X86PTE;
use crate::common::structs::mem::address::{PhysicalAddress, VirtualAddress};
use crate::{abstracts, sys};
use core::fmt::Debug;
use log::debug;
use x86_64::instructions::tlb;
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::{PageTableFlags, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

pub struct VmHALImpl;
impl VmHAL for VmHALImpl {
    fn current_addr() -> PhysicalAddress {
        let (active_page, _) = Cr3::read();
        active_page.start_address().as_u64() as _
    }

    fn activate(table_addr: PhysicalAddress) {
        let frame = PhysFrame::containing_address(PhysAddr::new(table_addr as _));
        unsafe {
            Cr3::write(frame, Cr3Flags::empty());
            debug!("switched page table to {:#x}", table_addr);
        }
    }

    fn flush_tlb(virt: Option<VirtualAddress>) {
        if let Some(virt) = virt {
            tlb::flush(VirtAddr::new(virt as _));
        } else {
            tlb::flush_all();
        }
    }

    // TODO: Implement this after Paging is implemented
    fn map_kernel_space(table_addr: PhysicalAddress, kernel_table_addr: PhysicalAddress) {
        let entry_range = 0x100..0x200; // 0xFFFF_8000_0000_0000 .. 0xFFFF_FFFF_FFFF_FFFF
        let dst_table = unsafe { core::slice::from_raw_parts_mut(sys::mem::address_space::phys_to_virt(table_addr) as *mut X86PTE, 512) };
        let src_table = unsafe { core::slice::from_raw_parts(sys::mem::address_space::phys_to_virt(kernel_table_addr) as *const X86PTE, 512) };
        for i in entry_range {
            dst_table[i] = src_table[i];
            if !dst_table[i].is_unused() {
                let current_flags = dst_table[i].flags();
                dst_table[i].set_flags(current_flags | PageTableFlags::GLOBAL.into(), false);
            }
        }
    }
}