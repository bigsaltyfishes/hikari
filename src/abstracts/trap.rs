use crate::common::structs::mem::address::VirtualAddress;
use crate::common::structs::mem::misc::MMUFlags;

#[derive(Debug, PartialEq, Eq)]
pub enum TrapReason {
    Syscall,
    Interrupt(usize),
    PageFault(VirtualAddress, MMUFlags),
    UndefinedInstruction,
    SoftwareBreakpoint,
    HardwareBreakpoint,
    UnalignedAccess,
    GernelFault(usize),
}