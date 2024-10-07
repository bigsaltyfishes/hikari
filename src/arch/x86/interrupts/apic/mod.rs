use crate::abstracts::interrupt::controller::{InterruptController, IrqHandler, IrqPolarity, IrqTriggerMode};
use crate::arch::x86::interrupts::apic::consts::{IOAPIC_INTERRUPT_VECTOR_NUM, IOAPIC_IRQ_RANGE, LAPIC_BASE, LAPIC_INTERRUPT_VECTOR_NUM, LAPIC_IRQ_RANGE};
use crate::arch::x86::interrupts::apic::ioapic::IoApicList;
use crate::arch::x86::interrupts::apic::lapic::LocalApic;
use crate::common::structs::interrupt::manager::{IrqError, IrqManager, IrqResult};
use alloc::vec;
use core::arch::asm;
use log::error;
use spin::Mutex;

// pub mod apic;
mod lapic;
mod consts;
mod ioapic;

pub struct Apic {
    io_apic_list: IoApicList,
    manager_ioapic: Mutex<IrqManager<{ IOAPIC_INTERRUPT_VECTOR_NUM }>>,
    manager_lapic: Mutex<IrqManager<{ LAPIC_INTERRUPT_VECTOR_NUM }>>,
}

impl Apic {
    pub fn new() -> Self {
        Self {
            io_apic_list: IoApicList::new(),
            manager_ioapic: Mutex::new(IrqManager::new(IOAPIC_IRQ_RANGE)),
            manager_lapic: Mutex::new(IrqManager::new(LAPIC_IRQ_RANGE)),
        }
    }

    fn with_ioapic<F>(&self, gsi: u32, op: F) -> IrqResult
    where
        F: FnOnce(&ioapic::IoApic) -> IrqResult,
    {
        if let Some(ioapic) = self.io_apic_list.find(gsi) {
            op(ioapic)
        } else {
            error!("IOAPIC not found for GSI {}", gsi);
            Err(IrqError::InvalidIrqVector)
        }
    }

    pub fn init_lapic_bsp() {
        unsafe { lapic::LocalApic::init_bsp() }
    }

    pub fn init_lapic_ap() {
        unsafe { lapic::LocalApic::init_ap() }
    }

    pub fn lapic<'a>() -> &'a mut LocalApic {
        unsafe { lapic::LocalApic::get() }
    }

    pub fn register_lapic_handler(&self, vector: usize, handler: IrqHandler) -> IrqResult {
        if vector >= LAPIC_BASE {
            self.manager_lapic.lock().register_handler(vector - LAPIC_BASE, handler)?;
            Ok(())
        } else {
            error!("Invalid LAPIC interrupt vector: {}", vector);
            Err(IrqError::InvalidIrqVector)
        }
    }

    pub fn unregister_lapic_handler(&self, vector: usize) -> IrqResult {
        if vector >= LAPIC_BASE {
            self.manager_lapic.lock().unregister_handler(vector - LAPIC_BASE)
        } else {
            error!("Invalid LAPIC interrupt vector: {}", vector);
            Err(IrqError::InvalidIrqVector)
        }
    }
}

impl InterruptController for Apic {
    fn is_valid_irq(&self, vector: usize) -> bool {
        self.io_apic_list.find(vector as _).is_some()
    }

    fn configure(&self, vector: usize, tm: IrqTriggerMode, pol: IrqPolarity) -> IrqResult {
        let gsi = vector as u32;
        self.with_ioapic(gsi, |ioapic| {
            ioapic.configure(gsi, tm, pol, LocalApic::bsp_id(), 0);
            Ok(())
        })
    }

    fn enable_interrupt(&self) -> IrqResult {
        unsafe { asm!("sti") };
        Ok(())
    }

    fn disable_interrupt(&self) -> IrqResult {
        unsafe { asm!("cli") };
        Ok(())
    }

    // TODO: Verify this
    fn is_interrupt_enabled(&self) -> bool {
        let mut rflags: usize = 0;
        unsafe {
            asm!(
            "pushfq",
            "pop {0}",
            out(reg) rflags
            )
        }
        rflags & 0x200 == 0
    }

    fn mask_irq(&self, vector: usize) -> IrqResult {
        let gsi = vector as u32;
        self.with_ioapic(gsi, |ioapic| {
            ioapic.toggle(gsi, false);
            Ok(())
        })
    }

    fn unmask_irq(&self, vector: usize) -> IrqResult {
        let gsi = vector as u32;
        self.with_ioapic(gsi, |ioapic| {
            ioapic.toggle(gsi, true);
            Ok(())
        })
    }

    fn register_irq_handler(&self, vector: usize, handler: IrqHandler) -> IrqResult {
        let gsi = vector as u32;
        self.with_ioapic(gsi, |apic| {
            let vector = apic.get_vector(gsi) as _;
            let vector = self.manager_ioapic.lock().register_handler(vector, handler)? as u8;
            apic.map_vector(gsi, vector);
            Ok(())
        })
    }

    fn unregister_irq_handler(&self, vector: usize) -> IrqResult {
        let gsi = vector as u32;
        self.with_ioapic(gsi, |apic| {
            let vector = apic.get_vector(gsi) as _;
            self.manager_ioapic.lock().unregister_handler(vector)?;
            apic.map_vector(gsi, 0);
            Ok(())
        })
    }

    fn handle_irq(&self, vector: usize) -> IrqResult {
        Self::lapic().eoi();
        let result = if vector >= LAPIC_BASE {
            self.manager_lapic.lock().handle_irq(vector - LAPIC_BASE)
        } else {
            self.manager_ioapic.lock().handle_irq(vector)
        };
        match result {
            Err(IrqError::InvalidIrqVector) => {
                error!("Invalid IRQ vector: {}", vector);
                Err(IrqError::InvalidIrqVector)
            }
            Err(IrqError::HandlerNotRegistered) => {
                error!("No registered handler for IRQ {}", vector);
                Ok(())
            }
            _ => {
                Ok(())
            }
        }
    }

    // TODO: Implement this with common HAL
    // fn apic_timer_enable(&self) {
    //     Self::lapic().enable_timer();
    // }
}