#[macro_export]
macro_rules! arch_specific_block {
    ($arch:expr, {$($item:item)*}) => {
        $(
            #[cfg(target_arch = $arch)]
            $item
        )*
    };
}

#[macro_export]
macro_rules! target_feature_required_block {
    ($feature:expr, $item:block) => {
        #[cfg(target_feature = $feature)]
        $item
    };
    ($feature:expr, $item:block, $else_block:block) => {
        #[cfg(target_feature = $feature)]
        $item
        #[cfg(not(target_feature = $feature))]
        $else_block
    };
}

arch_specific_block!("x86_64", {
    mod x86;

    pub use x86::ARCHITECTURE_MAX_DWARF_REGS;
    pub use x86::hal_impl;
});