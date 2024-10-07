use crate::info;
use x86::cpuid::CpuId;

pub const KERNEL_NAME: &str = "Hikari";
pub const KERNEL_MAJOR_VERSION: u8 = 0;
pub const KERNEL_MINOR_VERSION: u8 = 1;
pub const KERNEL_PATCH_VERSION: u8 = 0;
pub const KERNEL_LOCAL_VERSION: &str = "alpha";
pub const KERNEL_STACK_SIZE: usize = 1 << 12; // 4 MiB
pub const KERNEL_STACK_TRACE_FRAME_NUM: usize = 16;

pub fn print_sys_info() {
    info!(
        "{} v{}.{}.{}-{}",
        KERNEL_NAME,
        KERNEL_MAJOR_VERSION,
        KERNEL_MINOR_VERSION,
        KERNEL_PATCH_VERSION,
        KERNEL_LOCAL_VERSION
    );
    info!("Stack size: {} bytes", KERNEL_STACK_SIZE);
    info!("Stack trace frame number: {}", KERNEL_STACK_TRACE_FRAME_NUM);

    // Print the system information
    let cpuid = CpuId::new();
    info!("System Information:");
    info!("  - CPU: {} {}, Family: {}, Model: {}, Stepping: {}",
        cpuid.get_vendor_info().unwrap().as_str(),
        cpuid.get_processor_brand_string().unwrap().as_str(),
        cpuid.get_feature_info().unwrap().family_id(),
        cpuid.get_feature_info().unwrap().model_id(),
        cpuid.get_feature_info().unwrap().stepping_id()
    );
}