use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use x86_64::VirtAddr;

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("DOUBLE FAULT: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    let address = Cr2::read().unwrap_or(VirtAddr::new(0));

    let protv = error_code.contains(PageFaultErrorCode::PROTECTION_VIOLATION);
    let write = error_code.contains(PageFaultErrorCode::CAUSED_BY_WRITE);
    let user = error_code.contains(PageFaultErrorCode::USER_MODE);
    let malformed = error_code.contains(PageFaultErrorCode::MALFORMED_TABLE);
    let ins = error_code.contains(PageFaultErrorCode::INSTRUCTION_FETCH);
    panic!(
        "PAGE FAULT ({}{}{}{}{}at 0x{:x?})\n{:#?}",
        if protv { "protection-violation " } else { "" },
        if write { "read-only " } else { "" },
        if user { "user-mode " } else { "" },
        if malformed { "reserved " } else { "" },
        if ins { "fetch " } else { "" },
        address.as_u64(),
        stack_frame
    );
}

pub extern "x86-interrupt" fn divide_error_handler(
    stack_frame: InterruptStackFrame,
) {
    panic!("DIVIDE ERROR: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn invalid_opcode_handler(
    stack_frame: InterruptStackFrame,
) {
    panic!("INVALID OPCODE: {:#?}", stack_frame);
}

pub extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("GENERAL PROTECTION FAULT: {:#?}", stack_frame);
}