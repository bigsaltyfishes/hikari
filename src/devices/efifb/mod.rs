use alloc::vec::Vec;

use conquer_once::spin::OnceCell;
use lazy_static::lazy_static;
use log::info;
use spin::Mutex;

use crate::boot::BOOTINFO;
use crate::common::debug::console::Console;
use crate::common::debug::graphics::screen::{ColorMode, DisplayMode, Screen};

pub(crate) static SCREEN_INSTANCE: OnceCell<Mutex<Screen>> = OnceCell::uninit();

pub fn module_init() {
    if let Some(framebuffer) = BOOTINFO.framebuffer {
        if let Some(fb) = framebuffer.framebuffers().next() {
            // TODO: More ColorMode Support
            let color_mode = match (fb.red_mask_shift(), fb.green_mask_shift(), fb.blue_mask_shift()) {
                (0, 8, 16) => {
                    if fb.bpp() / 8 == 4 {
                        ColorMode::RGBA
                    } else {
                        ColorMode::RGB
                    }
                }
                (16, 8, 0) => {
                    if fb.bpp() / 8 == 4 {
                        ColorMode::BGRA
                    } else {
                        ColorMode::BGR
                    }
                }
                (0, 0, 0) => ColorMode::BLACK_WHITE,
                _ => {
                    info!("unsupported color mode: R({:#x}), G({:#x}), B({:#x})", fb.red_mask_shift(), fb.green_mask_shift(), fb.blue_mask_shift());
                    return;
                }
            };
            let screen_mode = DisplayMode::new(fb.width() as u32, fb.height() as u32, fb.bpp() as u32, fb.pitch() as u32, color_mode);
            info!("framebuffer: {}x{}x{} @ {:p}, mode: {:?}", screen_mode.width, screen_mode.height, screen_mode.depth, fb.addr(), color_mode);
            SCREEN_INSTANCE.init_once(|| Screen::new(
                unsafe { core::slice::from_raw_parts_mut(fb.addr(), (fb.pitch() * fb.height()) as usize) },
                screen_mode,
            ));
        }
    };
}