use alloc::vec::Vec;

use lazy_static::lazy_static;
use spin::Mutex;

use crate::common::structs::cell::Cell;
use crate::drivers::graphics::screen::Screen;

pub mod screen;
pub mod canvas;
pub mod framebuffer;

lazy_static!(
    pub static ref SCREEN_INSTANCES: Cell<Vec<Mutex<Screen<'static>>>> = Cell::new(Vec::new());
);
