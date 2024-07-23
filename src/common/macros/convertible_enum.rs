macro_rules! convertible_enum {
    (
        #[repr($repr:ident)]
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident = $value:expr),* $(,)*
        }
    ) => {
        #[repr($repr)]
        $(#[$meta])*
        $vis enum $name {
            $($variant = $value),*
        }

        impl TryFrom<$repr> for $name {
            type Error = $repr;

            fn try_from(v: $repr) -> Result<Self, Self::Error> {
                match v {
                    $( $value => Ok($name::$variant), )*
                    _ => Err(v),
                }
            }
        }

        paste::item! {
            impl $name {
                pub fn [<to_ $repr>] (&self) -> $repr {
                    unsafe { core::mem::transmute_copy(&self) }
                }
            }
        }
    };
}

pub(crate) use convertible_enum;