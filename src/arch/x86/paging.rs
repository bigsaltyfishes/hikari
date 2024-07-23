use spin::Mutex;
use x86_64::structures::paging::{OffsetPageTable, PageTable, Translate};
use x86_64::VirtAddr;

use crate::boot::BOOTINFO;
use crate::memory::paging::{PhysicalAddress, VirtualAddress};

static mut MAPPER: Option<Mutex<Mapper>> = None;

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

pub struct Mapper {
    inner: OffsetPageTable<'static>,
}

impl Mapper {
    pub unsafe fn init() {
        let virt_offset = VirtAddr::new(BOOTINFO.physics_mem_offset as u64);
        let l4_table = active_level_4_table(virt_offset);
        let mapper = OffsetPageTable::new(l4_table, virt_offset);
        MAPPER = Some(Mutex::new(Self {
            inner: mapper
        }));
    }

    pub fn translate(&self, addr: VirtualAddress) -> Option<PhysicalAddress> {
        let result = self.inner.translate_addr(VirtAddr::new(addr as u64));
        result.map(|physical_addr| physical_addr.as_u64() as PhysicalAddress)
    }
}

pub unsafe fn get_mapper() -> &'static Mutex<Mapper> {
    MAPPER.as_ref().unwrap()
}