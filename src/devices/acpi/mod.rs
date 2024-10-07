use crate::boot::BOOTINFO;
use crate::common::structs::cell::Cell;
use crate::info;
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};

static ACPI_TABLES: Cell<AcpiTables<AcpiHandlerImpl>> = Cell::uninit();

pub fn module_init() {
    if let Some(rsdp_address) = BOOTINFO.rsdp_address {
        info!("☞ Hikari ACPI Module");
        info!("Detected RSDP at 0x{:x}", rsdp_address);
        let rsdp_table = unsafe { AcpiTables::from_rsdp(AcpiHandlerImpl, rsdp_address) };
        if let Ok(rsdp_table) = rsdp_table {
            info!("ACPI Platform Revision: {}", rsdp_table.revision());
            ACPI_TABLES.init(rsdp_table);
            info!("ACPI Tables Initialized");
        }
    }
}

pub fn get_acpi_tables() -> &'static AcpiTables<AcpiHandlerImpl> {
    ACPI_TABLES.get_mut()
}

#[derive(Clone)]
pub struct AcpiHandlerImpl;

impl AcpiHandler for AcpiHandlerImpl {
    unsafe fn map_physical_region<T>(&self, physical_address: usize, size: usize) -> PhysicalMapping<Self, T> {
        // Limine have already mapped the mem region to high-half part of
        // the address space, so we can just use the physical address as the
        // virtual address.
        let virtual_address = if physical_address > BOOTINFO.physics_mem_offset {
            physical_address
        } else {
            physical_address + BOOTINFO.physics_mem_offset
        };
        PhysicalMapping::new(
            physical_address,
            core::ptr::NonNull::new(virtual_address as _).unwrap(),
            size,
            size,
            Self,
        )
    }

    fn unmap_physical_region<T>(region: &PhysicalMapping<Self, T>) {}
}