use core::fmt::Write;

use crate::common::debug::console::CONSOLE_INSTANCE;
use crate::devices::uart::u16550::SERIAL_WRITER;
use conquer_once::spin::OnceCell;
use log::{Level, Metadata, Record};
use spin::Mutex;

const LOG_BUFFER_SHIFT: u8 = 17;
const MAX_LOG_OUTPUT: usize = 2;

static mut LOGGER: OnceCell<Mutex<KernelLogger>> = OnceCell::uninit();

#[derive(Ord, PartialOrd, Eq, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

struct KernelLogger;

impl log::Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let level_str = match record.level() {
            Level::Trace => "\x1b[33mTRACE\x1b[37m",
            Level::Debug => "\x1b[33mDEBUG\x1b[37m",
            Level::Info => "\x1b[32mINFO\x1b[37m",
            Level::Warn => "\x1b[33mWARN\x1b[37m",
            Level::Error => "\x1b[31mERROR\x1b[37m",
        };

        let mut serial = SERIAL_WRITER.get().unwrap().lock();
        let _ = writeln!(serial, "[{}] {}", level_str, record.args());
        if let Some(console) = CONSOLE_INSTANCE.get() {
            let _ = writeln!(console.lock(), "[{}] {}", level_str, record.args());
        }
    }

    fn flush(&self) {
        if let Some(console) = CONSOLE_INSTANCE.get() {
            console.lock().clear();
        }
    }
}

pub fn module_init() {
    log::set_logger(&KernelLogger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}