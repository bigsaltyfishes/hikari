hal_fn_def! {
    pub mod cpu {
        /// Current CPU ID.
        pub fn cpu_id() -> u8;

        /// Current CPU frequency in MHz.
        pub fn cpu_frequency() -> u16;

        /// Shutdown/reboot the machine.
        pub fn reset() -> !;
    }
}