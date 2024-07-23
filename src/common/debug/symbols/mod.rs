use lazy_static::lazy_static;
use rustc_demangle::Demangle;

use crate::boot::BOOTINFO;

lazy_static!(
    pub static ref KERNEL_SYMBOLS: KernelSymbols = KernelSymbols::new();
);

#[derive(Debug, Eq, PartialEq)]
struct ElfSection(pub u32);
impl ElfSection {
    pub const PROGBITS: Self = Self(1);
    pub const SYMTAB: Self = Self(2);
    pub const STRTAB: Self = Self(3);
}

#[repr(C)]
#[derive(Debug)]
struct Elf64Header {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
#[derive(Debug)]
struct Elf64SectionHeader {
    sh_name: u32,
    sh_type: u32,
    sh_flags: u64,
    sh_addr: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Symbol {
    st_name: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
    st_value: u64,
    st_size: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct Elf64Strtab<'a> {
    data: &'a [u8],
}

impl<'a> Elf64Strtab<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
        }
    }

    pub unsafe fn from_ptr(ptr: *const u8, len: usize) -> Self {
        unsafe {
            Self {
                data: core::slice::from_raw_parts(ptr, len),
            }
        }
    }

    pub fn get_at(&self, offset: u32) -> Option<&'a str> {
        for (i, c) in self.data[offset as usize..].iter().enumerate() {
            if c == &0 {
                if i == 0 {
                    return Some("");
                }
                return Some(unsafe { core::str::from_utf8_unchecked(&self.data[offset as usize..offset as usize + i]) });
            }
        }
        None
    }
}

pub struct KernelSymbols {
    symtab: &'static [Elf64Symbol],
    strtab: Elf64Strtab<'static>,
}

impl KernelSymbols {
    pub fn new() -> Self {
        let kernel_address = BOOTINFO.kernel_file_address;
        let kernel_elf_header = unsafe { &*(kernel_address as *const Elf64Header) };
        let sheet_headers_base = (kernel_address + kernel_elf_header.e_shoff as usize) as *const Elf64SectionHeader;
        let sheet_string_table_head = unsafe { &*sheet_headers_base.offset(kernel_elf_header.e_shstrndx as isize) };
        let sheet_string_table = unsafe { Elf64Strtab::from_ptr((kernel_address + sheet_string_table_head.sh_offset as usize) as *const u8, sheet_string_table_head.sh_size as usize) };
        let mut symtab = None;
        let mut strtab = None;
        for i in 0..kernel_elf_header.e_shnum {
            let sheet = unsafe { &*sheet_headers_base.offset(i as isize) };

            match ElfSection(sheet.sh_type) {
                ElfSection::SYMTAB => {
                    if let Some(".symtab") = sheet_string_table.get_at(sheet.sh_name) {
                        symtab = Some(unsafe { core::slice::from_raw_parts((kernel_address + sheet.sh_offset as usize) as *const Elf64Symbol, (sheet.sh_size / sheet.sh_entsize) as usize) });
                    }
                }
                ElfSection::STRTAB => {
                    if let Some(".strtab") = sheet_string_table.get_at(sheet.sh_name) {
                        strtab = Some(unsafe { core::slice::from_raw_parts((kernel_address + sheet.sh_offset as usize) as *const u8, sheet.sh_size as usize) });
                    }
                }
                _ => {}
            }
        }
        Self {
            symtab: symtab.unwrap(),
            strtab: Elf64Strtab::new(strtab.unwrap()),
        }
    }

    pub fn find_symbol(&self, addr: usize) -> Option<(Demangle, usize)> {
        for sym in self.symtab.iter() {
            if sym.st_value <= (addr as u64) && (addr as u64) < sym.st_value + sym.st_size {
                let name = self.strtab.get_at(sym.st_name).unwrap_or("?");
                let offset = addr - sym.st_value as usize;
                return Some((rustc_demangle::demangle(name), offset));
            }
        }
        None
    }
}