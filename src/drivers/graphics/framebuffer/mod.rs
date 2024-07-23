use crate::boot::BOOTINFO;
use crate::drivers::define_module;
use crate::drivers::graphics::screen::{ColorMode, DisplayMode, Screen};
use crate::drivers::graphics::SCREEN_INSTANCES;
use crate::info;

define_module!(
    name => "Simple Framebuffer",
    init => {
        if let Some(framebuffer) = BOOTINFO.framebuffer {
            let screen_instances = SCREEN_INSTANCES.get_mut();
            for fb in framebuffer.framebuffers() {
                // TODO: More ColorMode Support
                let color_mode = match (fb.red_mask_shift(), fb.green_mask_shift(), fb.blue_mask_shift()) {
                    (0, 8, 16) => {
                        if fb.bpp() / 8 == 4 {
                            ColorMode::RGBA
                        } else {
                            ColorMode::RGB
                        }
                    },
                    (16, 8, 0) => {
                        if fb.bpp() / 8 == 4 {
                            ColorMode::BGRA
                        } else {
                            ColorMode::BGR
                        }
                    },
                    (0, 0, 0) => ColorMode::BLACK_WHITE,
                    _ => {
                        info!("Unsupported color mode: R({:#x}), G({:#x}), B({:#x})", fb.red_mask_shift(), fb.green_mask_shift(), fb.blue_mask_shift());
                        continue;
                    },
                };
                let screen_mode = DisplayMode::new(fb.width() as u32, fb.height() as u32, fb.bpp() as u32, fb.pitch() as u32, color_mode);
                info!("Framebuffer: {}x{}x{} @ {:p}, mode: {:?}", screen_mode.width, screen_mode.height, screen_mode.depth, fb.addr(), color_mode);
                screen_instances.push(
                    Screen::new(
                        unsafe { core::slice::from_raw_parts_mut(fb.addr(), (fb.pitch() * fb.height()) as usize) },
                        screen_mode
                    )
                );
            }
        };
    }
);