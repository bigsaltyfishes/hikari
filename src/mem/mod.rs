use buddy_system_allocator::LockedHeap;
use core::alloc::Layout;
use limine::memory_map::EntryType;
use log::{info, warn};
use crate::boot::BOOTINFO;
use crate::mem::defs::PAGE_BITS;

use crate::mem::address::PhysicalAddress;

pub mod defs;
pub mod address;
pub mod frame;
pub mod paging;

/// We need some memory for early initialization
static mut EARLY_MEMORY: [u8; 2 * 1024 * 1024] = [0u8; 2 * 1024 * 1024];

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

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

/// Allocate a batch of frames
///
/// # Arguments
/// count: usize - The number of frames to allocate
/// align_log2: usize - The alignment of the frames
///
/// # Returns
/// Option<PhysicalAddress> - The physical address of the allocated frames
pub fn frame_alloc(count: usize, align_log2: usize) -> Option<PhysicalAddress> {
    let ptr = HEAP_ALLOCATOR
        .lock()
        .alloc(
            Layout::from_size_align(
                count << PAGE_BITS,
                1 << (align_log2 + PAGE_BITS),
            ).unwrap()
        ).ok()?;
    Some(ptr.as_ptr() as PhysicalAddress - BOOTINFO.physics_mem_offset)
}

/// Deallocate a frame
///
/// # Arguments
/// ptr: PhysicalAddress - The physical address of the frame to deallocate
pub fn frame_dealloc(ptr: PhysicalAddress) {
    HEAP_ALLOCATOR
        .lock()
        .dealloc(
            unsafe {
                core::ptr::NonNull::new_unchecked((ptr + BOOTINFO.physics_mem_offset) as *mut u8)
            },
            Layout::from_size_align(
                1 << PAGE_BITS,
                1 << PAGE_BITS,
            ).unwrap(),
        );
}