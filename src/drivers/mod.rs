pub mod uart;
pub mod graphics;

macro_rules! define_module {
    (name => $name:expr, init => {$($init_stmts:stmt;)+}) => {
        pub fn module_init() -> crate::drivers::KernelModule {
            fn module_init_inner() {
                $($init_stmts)+
            }
            crate::drivers::KernelModule {
                name: $name,
                init: module_init_inner,
            }
        }
    };
}

pub(crate) use define_module;

pub struct KernelModule {
    pub name: &'static str,
    pub init: fn(),
}

pub fn early_load_drivers() {
    uart::module_init();
}