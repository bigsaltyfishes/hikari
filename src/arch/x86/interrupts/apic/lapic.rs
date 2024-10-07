use super::consts::{APIC_ERROR_INTERRUPT, APIC_SPURIOUS_INTERRUPT, APIC_TIMER_INTERRUPT};
use crate::abstracts::memory::address::AddressSpaceHAL;
use crate::sys;
use x2apic::lapic::{xapic_base, LocalApic as LocalApicInner, LocalApicBuilder, TimerDivide, TimerMode};

static mut LAPIC: Option<LocalApic> = None;
static mut BSP_ID: Option<u8> = None;

pub struct LocalApic {
    inner: LocalApicInner,
}

impl LocalApic {
    pub unsafe fn get<'a>() -> &'a mut LocalApic {
        LAPIC.as_mut().expect("Local APIC is not initialized")
    }

    pub unsafe fn init_bsp() {
        let base_vaddr = sys::mem::address_space::phys_to_virt(xapic_base() as usize);
        let mut inner = LocalApicBuilder::new()
            .timer_vector(APIC_TIMER_INTERRUPT)
            .error_vector(APIC_ERROR_INTERRUPT)
            .spurious_vector(APIC_SPURIOUS_INTERRUPT)
            .set_xapic_base(base_vaddr as _)
            .build()
            .unwrap_or_else(|err| panic!("Failed to initialize Local APIC: {:?}", err));
        inner.enable();

        assert!(inner.is_bsp());
        BSP_ID = Some((inner.id() >> 24) as u8);
        LAPIC = Some(LocalApic { inner });
    }

    pub unsafe fn init_ap() {
        Self::get().inner.enable();
    }

    pub fn bsp_id() -> u8 {
        unsafe { BSP_ID.expect("BSP is not initialized") }
    }

    pub fn id(&mut self) -> u8 {
        unsafe { (self.inner.id() >> 24) as u8 }
    }

    pub fn eoi(&mut self) {
        unsafe { self.inner.end_of_interrupt() }
    }

    pub fn disable_timer(&mut self) {
        unsafe { self.inner.disable_timer() }
    }

    pub fn enable_timer(&mut self) {
        unsafe { self.inner.enable_timer() }
    }

    pub fn set_timer_mode(&mut self, mode: TimerMode) {
        unsafe { self.inner.set_timer_mode(mode) }
    }

    pub fn set_timer_divide(&mut self, divide: TimerDivide) {
        unsafe { self.inner.set_timer_divide(divide) }
    }

    pub fn set_timer_initial(&mut self, initial: u32) {
        unsafe { self.inner.set_timer_initial(initial) }
    }
}