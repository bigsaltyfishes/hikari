#[macro_export]
macro_rules! registers {
    (
        registers => (
            $( $reg:ident ( $val:expr ) ),* $(,)?
        ),
        implement => {
            $( $item:tt )*
        }
    ) => {
        $crate::common::macros::convertible_enum! {
            #[repr(usize)]
            #[derive(Debug, Copy, Clone)]
            pub enum Registers {
                $(
                    $reg = $val,
                )*
            }
        }

        impl $crate::common::structs::register::Register for Registers {
            fn name(&self) -> &'static str {
                match self {
                    $(
                        Self::$reg => stringify!($reg),
                    )*
                }
            }

            $( $item )*
        }
    }
}

pub trait Register {
    fn get(&self) -> Option<usize>;
    unsafe fn set(&self, value: usize) -> Result<(), ()>;
    fn name(&self) -> &'static str;
}