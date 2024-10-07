use core::cell::RefCell;
use spin::Mutex;

use crate::common::debug::graphics::canvas::{Color, SimpleCanvas};

#[derive(Debug, Clone, Copy)]
pub struct ColorMode {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: Option<u8>,
}

impl ColorMode {
    pub const RGB: Self = Self {
        red: 0,
        green: 1,
        blue: 2,
        alpha: None,
    };

    pub const RGBA: Self = Self {
        red: 0,
        green: 1,
        blue: 2,
        alpha: Some(3),
    };

    pub const BGR: Self = Self {
        red: 2,
        green: 1,
        blue: 0,
        alpha: None,
    };

    pub const BGRA: Self = Self {
        red: 2,
        green: 1,
        blue: 0,
        alpha: Some(3),
    };

    pub const ARGB: Self = Self {
        red: 1,
        green: 2,
        blue: 3,
        alpha: Some(0),
    };

    pub const ABGR: Self = Self {
        red: 3,
        green: 2,
        blue: 1,
        alpha: Some(0),
    };

    pub const BLACK_WHITE: Self = Self {
        red: 0,
        green: 0,
        blue: 0,
        alpha: None,
    };
}

#[derive(Clone, Copy)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub pitch: u32,
    pub color_mode: ColorMode,
}

impl DisplayMode {
    pub fn new(width: u32, height: u32, depth: u32, pitch: u32, color_mode: ColorMode) -> Self {
        assert!(depth > 0, "Depth must be greater than 0");
        assert!(depth <= 32, "Depth must be less than or equal to 32");
        assert_eq!(depth % 8, 0, "Depth must be a multiple of 8");
        Self {
            width,
            height,
            depth,
            pitch,
            color_mode,
        }
    }
}

pub struct Screen<'a> {
    framebuffer: RefCell<&'a mut [u8]>,
    pub mode: DisplayMode,
}

impl<'a> Screen<'a> {
    pub fn new(framebuffer: &'a mut [u8], mode: DisplayMode) -> Mutex<Self> {
        Mutex::new(Self {
            framebuffer: RefCell::new(framebuffer),
            mode,
        })
    }

    pub fn get_mode(&self) -> &DisplayMode {
        &self.mode
    }
}

impl<'a> SimpleCanvas for Screen<'a> {
    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let bytes_per_pixel = self.mode.depth / 8;
        let framebuffer = self.framebuffer.borrow_mut();
        if x < self.mode.width && y < self.mode.height {
            let index = ((y * self.mode.width + x) * bytes_per_pixel) as usize;
            let mut color = Color::BLACK;
            for i in 0..bytes_per_pixel as u8 {
                if i == self.mode.color_mode.red {
                    color.red = framebuffer[index + i as usize];
                } else if i == self.mode.color_mode.green {
                    color.green = framebuffer[index + i as usize];
                } else if i == self.mode.color_mode.blue {
                    color.blue = framebuffer[index + i as usize];
                } else if Some(i) == self.mode.color_mode.alpha {
                    color.alpha = framebuffer[index + i as usize];
                }
            }
            color
        } else {
            Color::BLACK
        }
    }

    fn draw_pixel(&mut self, x: u32, y: u32, color: &Color) {
        if x >= self.mode.width || y >= self.mode.height {
            return;
        }
        let mut framebuffer = self.framebuffer.borrow_mut();
        let bytes_per_pixel = self.mode.depth / 8;
        let offset = (y * self.mode.pitch + x * bytes_per_pixel) as usize;
        let mut pixel: [u8; 4] = [0; 4];
        if bytes_per_pixel == 1 {
            pixel[0] = (color.red + color.green + color.blue) / 3;
        } else {
            for i in 0..bytes_per_pixel as u8 {
                if i == self.mode.color_mode.red {
                    pixel[i as usize] = color.red;
                } else if i == self.mode.color_mode.green {
                    pixel[i as usize] = color.green;
                } else if i == self.mode.color_mode.blue {
                    pixel[i as usize] = color.blue;
                } else if Some(i) == self.mode.color_mode.alpha {
                    pixel[i as usize] = color.alpha;
                } else {
                    pixel[i as usize] = 0;
                }
            }
        }
        framebuffer[offset..offset + bytes_per_pixel as usize].copy_from_slice(&pixel.as_slice()[0..bytes_per_pixel as usize]);
    }

    fn move_up(&mut self, dy: u32) {
        let mut framebuffer = self.framebuffer.borrow_mut();
        let offset = (dy * self.mode.pitch) as usize;
        let size = framebuffer.len() - offset;
        framebuffer.copy_within(offset.., 0);
        framebuffer[size..].fill(0);
    }

    fn clear(&mut self) {
        let mut framebuffer = self.framebuffer.borrow_mut();
        framebuffer.fill(0)
    }
}