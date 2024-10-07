use crate::common::structs::mem::address::VirtualAddress;
use numeric_enum_macro::numeric_enum;

#[derive(Debug)]
pub enum PagingError {
    NoMemory,
    NotMapped,
    AlreadyMapped,
    UnsupportedPageSize,
}

/// Address translation result.
pub type PagingResult<T = ()> = Result<T, PagingError>;

/// The [`PagingError::NotMapped`] can be ignored.
pub trait IgnoreNotMappedErr {
    /// If self is `Err(PagingError::NotMapped`, ignores the error and returns
    /// `Ok(())`, otherwise remain unchanged.
    fn ignore(self) -> PagingResult;
}

impl<T> IgnoreNotMappedErr for PagingResult<T> {
    fn ignore(self) -> PagingResult {
        match self {
            Ok(_) | Err(PagingError::NotMapped) => Ok(()),
            Err(e) => Err(e),
        }
    }
}


numeric_enum!(
    #[repr(usize)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum PageSize {
        Size4K = 0x1000,
        Size2M = 0x20_0000,
        Size1G = 0x4000_0000,
    }
);

// TODO: It seems some func have implemented by mem::address,
//       maybe we can remove them from here?
impl PageSize {
    pub const fn align_up(self, addr: usize) -> usize {
        self.align_down(addr + self as usize - 1)
    }
    pub const fn align_down(self, addr: usize) -> usize {
        addr & !(self as usize - 1)
    }
    pub const fn page_offset(self, addr: usize) -> usize {
        addr & (self as usize - 1)
    }
    pub const fn is_aligned(self, addr: usize) -> bool {
        self.page_offset(addr) == 0
    }
    pub const fn page_count(self, size: usize) -> usize {
        self.align_up(size) / (self as usize)
    }
    pub const fn is_huge(self) -> bool {
        matches!(self, PageSize::Size2M | PageSize::Size1G)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Page {
    pub virt_addr: VirtualAddress,
    pub size: PageSize,
}

impl Page {
    pub fn new_aligned(virt_addr: VirtualAddress, size: PageSize) -> Self {
        debug_assert!(size.is_aligned(virt_addr));
        Self {
            virt_addr,
            size,
        }
    }
}
