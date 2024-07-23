pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub trait PageMapper {
    fn translate(&self, addr: VirtualAddress) -> Option<PhysicalAddress>;
}