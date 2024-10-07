use crate::boot::BOOTINFO;
use buddy_system_allocator::LockedHeap;
use limine::memory_map::EntryType;
use log::{info, warn};

#[global_allocator]
pub static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

pub fn module_init() {
    let memory_map_response = BOOTINFO.memory_map;
    info!("Memory Map:");
    for entry in memory_map_response.entries() {
        match entry.entry_type {
            EntryType::USABLE => {
                let hole = entry.length % 4096;
                let begin = entry.base as usize + BOOTINFO.physics_mem_offset;
                let end = begin + (entry.length - hole) as usize;
                unsafe {
                    HEAP_ALLOCATOR.lock().add_to_heap(begin, end);
                }
                info!("  Usable Memory:    0x{:x} - 0x{:x} ({} bytes)", entry.base, entry.base + entry.length - hole, entry.length - hole);
                if hole > 0 {
                    warn!("  Memory Hole:      0x{:x} - 0x{:x} ({} bytes)", entry.base + entry.length - hole, entry.base + entry.length, hole);
                }
            }
            EntryType::BAD_MEMORY => {
                warn!("  Bad Memory:       0x{:x} - 0x{:x} ({} bytes)", entry.base, entry.base + entry.length, entry.length);
            }
            EntryType::ACPI_NVS => {
                info!("  ACPI NVS:         0x{:x} - 0x{:x} ({} bytes)", entry.base, entry.base + entry.length, entry.length);
            }
            EntryType::ACPI_RECLAIMABLE => {
                info!("  ACPI Reclaimable: 0x{:x} - 0x{:x} ({} bytes)", entry.base, entry.base + entry.length, entry.length);
            }
            _ => {
                info!("  Reserved Memory:  0x{:x} - 0x{:x} ({} bytes)", entry.base, entry.base + entry.length, entry.length);
            }
        }
    }

    info!("Available Memory Size: 0x{:x} bytes", HEAP_ALLOCATOR.lock().stats_total_bytes());
}