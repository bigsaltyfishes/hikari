use crate::abstracts::memory::address::AddressSpaceHAL;
use crate::abstracts::memory::vm::VmHAL;
use crate::common::structs::mem::address::{PhysicalAddress, VirtualAddress};
use crate::common::structs::mem::frame::PhysicalFrame;
use crate::common::structs::mem::misc::MMUFlags;
use crate::common::structs::mem::paging::{Page, PageSize, PagingError, PagingResult};
use crate::{abstracts, sys};
use alloc::vec::Vec;
use core::fmt::Debug;
use core::marker::PhantomData;
use log::{debug, trace};

pub const ENTRY_COUNT: usize = 512;

/// A generic page table trait
///
/// This trait is used to define the interface for a generic page table.
pub trait GenericPageTable: Sync + Send {
    /// Get the physical address of the root table
    fn table_phys(&self) -> PhysicalAddress;

    /// Map a page to a frame with specified flags
    fn map(&mut self, page: Page, phys: PhysicalAddress, flags: MMUFlags) -> PagingResult;

    /// Unmap a page
    fn unmap(&mut self, virt: VirtualAddress) -> PagingResult<(PhysicalAddress, PageSize)>;

    /// Update physical address or flags of a page
    fn update(&mut self, virt: VirtualAddress, phys_addr: Option<PhysicalAddress>, flags: Option<MMUFlags>) -> PagingResult;

    /// Query the physical address and flags of a page
    fn query(&mut self, virt: VirtualAddress) -> PagingResult<(PhysicalAddress, MMUFlags, PageSize)>;

    /// Map a range of physical mem to virtual mem with specified flags
    fn map_range(&mut self, start_virt: VirtualAddress, start_phys: PhysicalAddress, size: usize, flags: MMUFlags) -> PagingResult {
        assert!(PageSize::Size4K.is_aligned(start_virt));
        assert!(PageSize::Size4K.is_aligned(start_phys));
        assert!(PageSize::Size4K.is_aligned(size));
        debug!("Mapping range: {:x?} -> {:x?} (size: {:x?}, flags: {:?})", start_virt, start_phys, size, flags);
        let mut mapped_size = 0usize;
        if !flags.contains(MMUFlags::HUGE_PAGE) {
            while mapped_size < size {
                let page = Page::new_aligned(start_virt + mapped_size, PageSize::Size4K);
                let phys = start_phys + mapped_size;
                self.map(page, phys, flags)?;
                mapped_size += PageSize::Size4K as usize;
            }
        } else {
            while mapped_size < size {
                let start_virt = start_virt + mapped_size;
                let start_phys = start_phys + mapped_size;
                let page_size = if size - mapped_size >= PageSize::Size1G as usize
                    && PageSize::Size1G.is_aligned(start_virt)
                    && PageSize::Size1G.is_aligned(start_phys)
                {
                    PageSize::Size1G
                } else if size - mapped_size >= PageSize::Size2M as usize
                    && PageSize::Size2M.is_aligned(start_virt)
                    && PageSize::Size2M.is_aligned(start_phys)
                {
                    PageSize::Size2M
                } else {
                    PageSize::Size4K
                };

                let page = Page::new_aligned(start_virt, page_size);
                self.map(page, start_phys, flags)?;
                mapped_size += page_size as usize;
            }
        }
        Ok(())
    }

    /// Unmap a range of virtual mem
    fn unmap_range(&mut self, start_virt: VirtualAddress, size: usize) -> PagingResult {
        assert!(PageSize::Size4K.is_aligned(start_virt));
        assert!(PageSize::Size4K.is_aligned(size));
        debug!("Unmapping range: {:x?} (size: {:x?})", start_virt, size);
        let mut unmapped_size = 0usize;
        while unmapped_size < size {
            let page_size = match self.unmap(start_virt + unmapped_size) {
                Ok((_, page_size)) => page_size,
                Err(PagingError::NotMapped) => {
                    PageSize::Size4K
                }
                Err(err) => return Err(err),
            };
            unmapped_size += page_size as usize;
        }
        Ok(())
    }
}

pub trait GenericPTE: Debug + Clone + Copy + Sync + Send {
    /// Returns the physical address mapped by this entry.
    fn addr(&self) -> PhysicalAddress;
    /// Returns the flags of this entry.
    fn flags(&self) -> MMUFlags;
    /// Returns whether this entry is zero.
    fn is_unused(&self) -> bool;
    /// Returns whether this entry flag indicates present.
    fn is_present(&self) -> bool;
    /// Returns whether this entry maps to a huge frame (or it's a terminal entry).
    fn is_leaf(&self) -> bool;
    /// Set flags for all types of entries.
    fn set_flags(&mut self, flags: MMUFlags, is_huge: bool);
    /// Set physical address for terminal entries.
    fn set_addr(&mut self, phys: PhysicalAddress);
    /// Set physical address and flags for intermediate table entries.
    fn set_table(&mut self, phys: PhysicalAddress);
    /// Set this entry to zero.
    fn clear(&mut self);
}

pub struct PageTableImpl<const LEVEL: usize, PTE: GenericPTE> {
    root: PhysicalFrame,
    intrm_tables: Vec<PhysicalFrame>,
    _phantom: PhantomData<PTE>,
}

impl<const LEVEL: usize, PTE: GenericPTE> PageTableImpl<LEVEL, PTE> {
    unsafe fn from_root(phys_addr: PhysicalAddress) -> Self {
        Self {
            root: PhysicalFrame::from_phys(phys_addr),
            intrm_tables: Vec::new(),
            _phantom: PhantomData,
        }
    }

    fn grow(&mut self) -> Option<PhysicalAddress> {
        let frame = PhysicalFrame::new_with_zero()?;
        let phys_addr = frame.phys_addr();
        self.intrm_tables.push(frame);
        Some(phys_addr)
    }

    // TODO: Check the behavior of this function
    fn get_entry_mut_inner(&mut self, virt: VirtualAddress, target_page_size: Option<PageSize>, auto_grow: bool) -> PagingResult<(&mut PTE, PageSize)> {
        let mut phys_to_mut = |phys: PhysicalAddress| {
            let ptr = sys::mem::address_space::phys_to_virt(phys) as *mut PTE;
            unsafe { core::slice::from_raw_parts_mut(ptr, ENTRY_COUNT) }
        };

        let mut current_frame: &mut [PTE] = phys_to_mut(self.root.phys_addr());

        for level in (0..LEVEL).rev() {
            let index = ((virt >> (12 + 9 * level)) & (ENTRY_COUNT - 1)) as usize;
            let page_size: usize = 1 << (12 + 9 * level);
            let entry = &mut current_frame[index];
            if entry.is_leaf() || page_size == target_page_size.unwrap_or(PageSize::Size4K) as usize {
                return Ok((entry, PageSize::try_from(page_size).map_err(|_| PagingError::UnsupportedPageSize)?));
            }

            if entry.is_unused() && auto_grow {
                let phys_addr = self.grow().ok_or(PagingError::NoMemory)?;
                entry.set_table(phys_addr);
            }

            if entry.is_present() {
                current_frame = phys_to_mut(entry.addr());
            } else {
                return Err(PagingError::NotMapped);
            }
        }
        unreachable!();
    }

    fn get_entry_mut(&mut self, virt: VirtualAddress) -> PagingResult<(&mut PTE, PageSize)> {
        self.get_entry_mut_inner(virt, None, false)
    }

    fn get_entry_mut_or_create(&mut self, page: Page) -> PagingResult<(&mut PTE, PageSize)> {
        self.get_entry_mut_inner(page.virt_addr, Some(page.size), true)
    }

    pub(crate) unsafe fn activate(&self) {
        sys::mem::vmm::activate(self.root.phys_addr());
    }
}

impl<const LEVEL: usize, PTE: GenericPTE> PageTableImpl<LEVEL, PTE> {
    pub fn new() -> Self {
        let root = PhysicalFrame::new_with_zero().expect("Failed to allocate a frame for the root table");
        Self {
            root,
            intrm_tables: Vec::new(),
            _phantom: PhantomData,
        }
    }

    pub fn clone_kernel_space(&self) -> Self {
        let pt = Self::new();
        sys::mem::vmm::map_kernel_space(pt.table_phys(), self.table_phys());
        pt
    }

    pub fn from_active() -> Self {
        let root = unsafe { sys::mem::vmm::current_addr() };
        unsafe { Self::from_root(root) }
    }
}

impl<const LEVEL: usize, PTE: GenericPTE> GenericPageTable for PageTableImpl<LEVEL, PTE> {
    fn table_phys(&self) -> PhysicalAddress {
        self.root.phys_addr()
    }

    fn map(&mut self, page: Page, phys: PhysicalAddress, flags: MMUFlags) -> PagingResult {
        let (entry, _) = self.get_entry_mut_or_create(page)?;
        entry.set_addr(page.size.align_down(phys));
        entry.set_flags(flags, page.size.is_huge());
        sys::mem::vmm::flush_tlb(Some(page.virt_addr));
        trace!("Mapped: {:x?} -> {:x?} (flags: {:?}, table: {:#x?})", page.virt_addr, phys, flags, self.table_phys());
        Ok(())
    }

    fn unmap(&mut self, virt: VirtualAddress) -> PagingResult<(PhysicalAddress, PageSize)> {
        let (entry, page_size) = self.get_entry_mut(virt)?;
        if entry.is_unused() {
            return Err(PagingError::NotMapped);
        }
        let phys = entry.addr();
        entry.clear();
        sys::mem::vmm::flush_tlb(Some(virt));
        trace!("Unmapped: {:x?} (table: {:#x?})", virt, self.table_phys());
        Ok((phys, page_size))
    }

    fn update(&mut self, virt: VirtualAddress, phys_addr: Option<PhysicalAddress>, flags: Option<MMUFlags>) -> PagingResult {
        let (entry, size) = self.get_entry_mut(virt)?;
        if let Some(phys) = phys_addr {
            entry.set_addr(phys);
        }
        if let Some(flags) = flags {
            entry.set_flags(flags, size.is_huge());
        }
        sys::mem::vmm::flush_tlb(Some(virt));
        trace!("Updated: {:x?} (flags: {:?}, table: {:#x?})", virt, flags, self.table_phys());
        Ok(())
    }

    fn query(&mut self, virt: VirtualAddress) -> PagingResult<(PhysicalAddress, MMUFlags, PageSize)> {
        let (entry, size) = self.get_entry_mut(virt)?;
        if entry.is_unused() {
            return Err(PagingError::NotMapped);
        }
        let offset = size.page_offset(virt);
        Ok((entry.addr() + offset, entry.flags(), size))
    }
}