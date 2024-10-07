use crate::arch::x86::init::X86init;
use crate::sys::init::kernel_init;
use crate::kmain;

#[no_mangle]
extern "C" fn _start() -> ! {
    // Initialize CPU
    kernel_init::<X86init>();

    unsafe { kmain() }
}