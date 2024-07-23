use lazy_static::lazy_static;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, HhdmRequest, KernelAddressRequest, KernelFileRequest, MemoryMapRequest, StackSizeRequest};
use limine::response::{FramebufferResponse, MemoryMapResponse};

use crate::kinfo::KERNEL_STACK_SIZE;

macro_rules! limine_request {
    ($($name:ident => $vis:vis($ty:ty, $expr:expr)),* $(,)?) => {
        $(
            #[used]
            #[link_section = ".requests"]
            $vis static $name: $ty = $expr;
        )*
    };
    ($($name:ident => ($ty:ty, $expr:expr)),* $(,)?) => {
        $(
            #[used]
            #[link_section = ".requests"]
            static $name: $ty = $expr;
        )*
    };
}

limine_request!(
    BASE_REVISION => (BaseRevision, BaseRevision::new()),
    MEMORY_MAP_REQUEST => (MemoryMapRequest, MemoryMapRequest::new()),
    FRAMEBUFFER_REQUEST => (FramebufferRequest, FramebufferRequest::new()),
    KERNEL_ADDRESS_REQUEST => (KernelAddressRequest, KernelAddressRequest::new()),
    KERNEL_FILE_REQUEST => (KernelFileRequest, KernelFileRequest::new()),
    HHDM_REQUEST => (HhdmRequest, HhdmRequest::new()),
    STACK_SIZE_REQUEST => (StackSizeRequest, StackSizeRequest::new().with_size(KERNEL_STACK_SIZE as u64)),
);

lazy_static!(
    pub static ref BOOTINFO: BootInformation = BootInformation::new();
);

pub struct BootInformation {
    pub bootloader_version: &'static BaseRevision,
    pub memory_map: &'static MemoryMapResponse,
    pub framebuffer: Option<&'static FramebufferResponse>,
    pub kernel_address: usize,
    pub kernel_file_address: usize,
    pub kernel_file_length: usize,
    pub physics_mem_offset: usize,
}

impl BootInformation {
    pub fn new() -> Self {
        Self {
            bootloader_version: &BASE_REVISION,
            memory_map: MEMORY_MAP_REQUEST.get_response().unwrap(),
            framebuffer: FRAMEBUFFER_REQUEST.get_response(),
            kernel_address: KERNEL_ADDRESS_REQUEST.get_response().unwrap().physical_base() as usize,
            kernel_file_address: KERNEL_FILE_REQUEST.get_response().unwrap().file().addr() as usize,
            kernel_file_length: KERNEL_FILE_REQUEST.get_response().unwrap().file().size() as usize,
            physics_mem_offset: HHDM_REQUEST.get_response().unwrap().offset() as usize,
        }
    }
}