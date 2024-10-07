// Re-export the HAL implementation.
pub use crate::arch::hal_impl::memory::vm::VmHALImpl as implement;

/// Virtual Memory Hardware Abstraction Layer
pub trait VmHAL {
    /// Read the current page table base address.
    fn current_addr() -> crate::mem::address::PhysicalAddress;

    /// Activate the page table.
    fn activate(table_addr: crate::mem::address::PhysicalAddress);

    /// Flush the TLB.
    fn flush_tlb(virt: Option<crate::mem::address::VirtualAddress>);

    /// Map sys space to target page table.
    /// This will clone sys space entries (top level only) to target page table.
    ///
    /// # Arguments
    /// table_addr: PhysicalAddress - The target page table address.
    /// kernel_table_addr: PhysicalAddress - The table containing sys space entries.
    fn map_kernel_space(table_addr: crate::mem::address::PhysicalAddress, kernel_table_addr: crate::mem::address::PhysicalAddress);
}