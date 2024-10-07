use crate::abstracts::memory::table::{GenericPTE, PageTableImpl};
use crate::common::structs::mem::address::PhysicalAddress;
use crate::common::structs::mem::misc::{CachePolicy, MMUFlags};
use core::fmt::{Debug, Formatter};
use x86_64::structures::paging::PageTableFlags;

pub use X86PTE as PageTableEntry;
pub type PageTable = PageTableImpl<4, PageTableEntry>;

const PHYS_ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct X86PTE(u64);

impl GenericPTE for X86PTE {
    fn addr(&self) -> PhysicalAddress {
        (self.0 & PHYS_ADDR_MASK) as _
    }
    fn flags(&self) -> MMUFlags {
        PageTableFlags::from_bits_truncate(self.0).into()
    }
    fn is_unused(&self) -> bool {
        self.0 == 0
    }
    fn is_present(&self) -> bool {
        PageTableFlags::from_bits_truncate(self.0).contains(PageTableFlags::PRESENT)
    }
    fn is_leaf(&self) -> bool {
        PageTableFlags::from_bits_truncate(self.0).contains(PageTableFlags::HUGE_PAGE)
    }

    fn set_flags(&mut self, flags: MMUFlags, is_huge: bool) {
        let mut flags: PageTableFlags = flags.into();
        if is_huge {
            flags |= PageTableFlags::HUGE_PAGE;
        }
        self.0 = self.addr() as u64 | flags.bits();
    }
    fn set_addr(&mut self, phys: PhysicalAddress) {
        self.0 = (self.0 & !PHYS_ADDR_MASK) | (phys as u64 & PHYS_ADDR_MASK);
    }
    fn set_table(&mut self, phys: PhysicalAddress) {
        self.0 = (phys as u64 & PHYS_ADDR_MASK)
            | (PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE).bits();
    }
    fn clear(&mut self) {
        self.0 = 0
    }
}

impl Debug for X86PTE {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut f = f.debug_struct("X86PTE");
        f.field("raw", &self.0);
        f.field("addr", &self.addr());
        f.field("flags", &self.flags());
        f.finish()
    }
}

impl From<MMUFlags> for PageTableFlags {
    fn from(f: MMUFlags) -> Self {
        if f.is_empty() {
            return PageTableFlags::empty();
        }
        let mut flags = PageTableFlags::PRESENT;
        if f.contains(MMUFlags::WRITE) {
            flags |= PageTableFlags::WRITABLE;
        }
        if !f.contains(MMUFlags::EXECUTE) {
            flags |= PageTableFlags::NO_EXECUTE;
        }
        if f.contains(MMUFlags::USER) {
            flags |= PageTableFlags::USER_ACCESSIBLE;
        }
        let cache_policy = (f.bits() & 3) as u32;
        match CachePolicy::try_from(cache_policy) {
            Ok(CachePolicy::Cached) => {
                flags.remove(PageTableFlags::WRITE_THROUGH);
            }
            Ok(CachePolicy::Uncached) | Ok(CachePolicy::UncachedDevice) => {
                flags |= PageTableFlags::NO_CACHE | PageTableFlags::WRITE_THROUGH;
            }
            Ok(CachePolicy::WriteCombining) => {
                flags |= PageTableFlags::NO_CACHE | PageTableFlags::WRITE_THROUGH;
            }
            Err(_) => unreachable!("invalid cache policy"),
        }
        flags
    }
}

impl From<PageTableFlags> for MMUFlags {
    fn from(f: PageTableFlags) -> Self {
        let mut flags = MMUFlags::empty();
        if f.contains(PageTableFlags::WRITABLE) {
            flags |= MMUFlags::WRITE;
        }
        if !f.contains(PageTableFlags::NO_EXECUTE) {
            flags |= MMUFlags::EXECUTE;
        }
        if f.contains(PageTableFlags::USER_ACCESSIBLE) {
            flags |= MMUFlags::USER;
        }
        if f.contains(PageTableFlags::NO_CACHE | PageTableFlags::WRITE_THROUGH) {
            flags |= MMUFlags::CACHE_1;
        }
        flags
    }
}