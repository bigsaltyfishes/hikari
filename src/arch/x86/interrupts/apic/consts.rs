use core::ops::Range;


pub const IOAPIC_INTERRUPT_VECTOR_NUM: usize = 256;
pub const LAPIC_INTERRUPT_VECTOR_NUM: usize = 16;
pub const IOAPIC_BASE: usize = 0x20;
pub const LAPIC_BASE: usize = 0xf0;
pub const LAPIC_IRQ_RANGE: Range<usize> = 0..16;
pub const IOAPIC_IRQ_RANGE: Range<usize> = IOAPIC_BASE..LAPIC_BASE;

pub const APIC_TIMER_INTERRUPT: usize = LAPIC_BASE + 1;
pub const APIC_ERROR_INTERRUPT: usize = LAPIC_BASE + 2;
pub const APIC_SPURIOUS_INTERRUPT: usize = LAPIC_BASE + 3;
pub const APIC_TLB_FLUSH_INTERRUPT: usize = LAPIC_BASE + 4;