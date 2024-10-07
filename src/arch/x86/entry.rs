use crate::arch::x86::init::X86init;
use crate::kmain;
use crate::sys::init::kernel_init;

#[no_mangle]
extern "C" fn _start() -> ! {
    // Initialize CPU
    kernel_init::<X86init>();

    unsafe { kmain() }
}