use lazy_static::lazy_static;
use x86_64::set_general_handler;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::arch::x86::gdt;
use crate::arch::x86::interrupts::exceptions::{divide_error_handler, double_fault_handler, general_protection_fault_handler, invalid_opcode_handler, page_fault_handler};
use crate::info;
use crate::interrupt::INTERRUPT_MANAGER;

mod exceptions;

lazy_static!(
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        set_general_handler!(&mut idt, general_handler, 0..255);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
            idt.page_fault
                .set_handler_fn(page_fault_handler)
                .set_stack_index(gdt::PAGE_FAULT_IST_INDEX);
            idt.divide_error.set_handler_fn(divide_error_handler);
            idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
            idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        };

        idt
    };
);

pub fn module_init() {
    IDT.load();
    info!("IDT loaded");
}

fn general_handler(
    _stack_frame: InterruptStackFrame,
    index: u8,
    _error_code: Option<u64>,
) {
    unsafe { INTERRUPT_MANAGER.handle_interrupt(index as usize); }
}