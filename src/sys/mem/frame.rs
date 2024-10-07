use crate::boot::BOOTINFO;
use crate::common::structs::mem::address::PhysicalAddress;
use crate::common::structs::mem::misc::PAGE_BITS;
use crate::sys::mem::heap::HEAP_ALLOCATOR;
use core::alloc::Layout;

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