// Re-export the HAL implementation.
pub use crate::arch::hal_impl::memory::AddressSpaceHALImpl as address_space;
pub mod paging {
    pub use crate::arch::hal_impl::memory::table::PageTable;
}
pub use crate::arch::hal_impl::memory::vm::VmHALImpl as vmm;

pub mod frame;
pub mod heap;