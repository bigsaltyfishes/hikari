use core::fmt;
use core::fmt::Write;

use conquer_once::spin::OnceCell;
use spin::Mutex;

use crate::common::structs::buffer::IOBuffer;
use crate::common::structs::cell::Cell;

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

struct KernelLogger {
    buffer: Cell<Mutex<IOBuffer<{ 1 << LOG_BUFFER_SHIFT }>>>,
    outputs: [Cell<&'static Mutex<dyn Write>>; MAX_LOG_OUTPUT],
}

impl KernelLogger {
    pub fn init() {
        unsafe {
            LOGGER.init_once(move || {
                Mutex::new(Self {
                    buffer: Cell::uninit(),
                    outputs: core::array::from_fn(|_| Cell::default()),
                })
            })
        }
    }

    pub fn create_buffer(&self) {
        self.buffer.init(Mutex::new(IOBuffer::new()));
    }

    pub fn log(&self, level: LogLevel, args: fmt::Arguments) {
        let level_str = match level {
            LogLevel::Trace => "\x1b[33mTRACE\x1b[37m",
            LogLevel::Debug => "\x1b[33mDEBUG\x1b[37m",
            LogLevel::Info => "\x1b[32mINFO\x1b[37m",
            LogLevel::Warn => "\x1b[33mWARN\x1b[37m",
            LogLevel::Error => "\x1b[31mERROR\x1b[37m",
        };

        if self.buffer.is_init() {
            let mut buffer = self.buffer.get_mut().lock();
            writeln!(buffer, "[{}] {}", level_str, args).unwrap();
        }

        let print = |arg: fmt::Arguments| {
            for output in &self.outputs {
                if output.is_init() {
                    write!(output.get_mut().lock(), "{}", arg).unwrap();
                }
            }
        };

        print(format_args!("[{}] {}\n", level_str, args));
    }

    pub fn add_output(&self, output: &'static Mutex<dyn Write>) {
        for cell in &self.outputs {
            if !cell.is_init() {
                cell.init(output);
                if self.buffer.is_init() {
                    let buffer = self.buffer.get_mut().lock();
                    let (buf_0, buf_1) = buffer.as_slice();
                    if !buf_0.is_empty() {
                        write!(output.lock(), "{}", core::str::from_utf8(buf_0).unwrap()).unwrap();
                    }
                    if !buf_1.is_empty() {
                        write!(output.lock(), "{}", core::str::from_utf8(buf_1).unwrap()).unwrap();
                    }
                }
                return;
            }
        }
    }
}

pub fn module_init() {
    KernelLogger::init();
}

pub fn create_buffer() {
    unsafe {
        LOGGER.get().unwrap().lock().create_buffer();
    }
}

pub fn log(level: LogLevel, args: fmt::Arguments) {
    unsafe {
        LOGGER.get().unwrap().lock().log(level, args);
    }
}

pub fn add_log_output(output: &'static Mutex<dyn Write>) {
    unsafe {
        LOGGER.get().unwrap().lock().add_output(output);
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::common::debug::logger::log($crate::common::debug::logger::LogLevel::Info, format_args!($($arg)*))
    };
    () => {};
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::common::debug::logger::log($crate::common::debug::logger::LogLevel::Warn, format_args!($($arg)*))
    };
    () => {};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::common::debug::logger::log($crate::common::debug::logger::LogLevel::Error, format_args!($($arg)*))
    };
    () => {};
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::common::debug::logger::log($crate::common::debug::logger::LogLevel::Trace, format_args!($($arg)*))
    };
    () => {};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::common::debug::logger::log($crate::common::debug::logger::LogLevel::Debug, format_args!($($arg)*))
    };
    () => {};
}