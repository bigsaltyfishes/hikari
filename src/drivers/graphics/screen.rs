use alloc::vec;
use alloc::vec::Vec;

use spin::Mutex;

use crate::common::structs::cell::Cell;
use crate::drivers::graphics::canvas::{Canvas, Color};

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
    framebuffer: Cell<&'a mut [u8]>,
    pub mode: DisplayMode,
}

impl<'a> Screen<'a> {
    pub fn new(framebuffer: &'a mut [u8], mode: DisplayMode) -> Mutex<Self> {
        Mutex::new(Self {
            framebuffer: Cell::new(framebuffer),
            mode,
        })
    }

    pub fn get_mode(&self) -> &DisplayMode {
        &self.mode
    }
}

impl<'a> Canvas for Screen<'a> {
    fn get_pixel(&self, x: u32, y: u32) -> Color {
        let bytes_per_pixel = self.mode.depth / 8;
        let framebuffer = self.framebuffer.get_mut();
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
        let framebuffer = self.framebuffer.get_mut();
        let bytes_per_pixel = self.mode.depth / 8;
        let offset = (y * self.mode.pitch + x * bytes_per_pixel) as usize;
        let mut pixel = Vec::<u8>::with_capacity(bytes_per_pixel as usize);
        if bytes_per_pixel == 1 {
            pixel.push((color.red + color.green + color.blue) / 3);
        } else {
            for i in 0..bytes_per_pixel as u8 {
                if i == self.mode.color_mode.red {
                    pixel.push(color.red);
                } else if i == self.mode.color_mode.green {
                    pixel.push(color.green);
                } else if i == self.mode.color_mode.blue {
                    pixel.push(color.blue);
                } else if Some(i) == self.mode.color_mode.alpha {
                    pixel.push(color.alpha);
                } else {
                    pixel.push(0);
                }
            }
        }
        framebuffer[offset..offset + bytes_per_pixel as usize].copy_from_slice(pixel.as_slice());
    }

    fn draw_line(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, color: &Color) {
        let dx = (x2 as i32 - x1 as i32).abs();
        let dy = (y2 as i32 - y1 as i32).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        let mut x = x1 as i32;
        let mut y = y1 as i32;
        let steps = dx.max(dy) as u32;

        for t in 0..=steps {
            let blended_color = color.blend(color, t, steps);
            self.draw_pixel(x as u32, y as u32, &blended_color);

            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx;
            }
            if e2 < dy {
                err += dx;
                y += sy;
            }
        }
    }

    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: &Color) {
        self.draw_line(x, y, x + width, y, color);
        self.draw_line(x, y + height, x + width, y + height, color);
        self.draw_line(x, y, x, y + height, color);
        self.draw_line(x + width, y, x + width, y + height, color);
    }

    fn draw_circle(&mut self, x: u32, y: u32, radius: u32, color: &Color) {
        fn distance(x: i32, y: i32) -> u32 {
            (x * x + y * y).isqrt() as u32
        }
        let x0 = x as i32;
        let y0 = y as i32;
        let radius = radius as i32;
        let mut x = radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            let max_distance = distance(radius, 0);

            let positions = [
                (x0 + x, y0 + y),
                (x0 + y, y0 + x),
                (x0 - y, y0 + x),
                (x0 - x, y0 + y),
                (x0 - x, y0 - y),
                (x0 - y, y0 - x),
                (x0 + y, y0 - x),
                (x0 + x, y0 - y),
            ];

            for &(px, py) in &positions {
                let dist = distance(px - x0, py - y0);
                let blended_color = color.blend(color, dist, max_distance);
                self.draw_pixel(px as u32, py as u32, &blended_color);
            }

            if err <= 0 {
                y += 1;
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: &Color) {
        for i in 0..width {
            for j in 0..height {
                self.draw_pixel(x + i, y + j, color);
            }
        }
    }

    fn fill_circle(&mut self, x: u32, y: u32, radius: u32, color: &Color) {
        let x0 = x as i32;
        let y0 = y as i32;
        let radius = radius as i32;

        for y in -radius..=radius {
            let x_span = (radius * radius - y * y).isqrt();
            for x in -x_span..=x_span {
                let distance = (x * x + y * y).isqrt() as u32;
                let blended_color = color.blend(color, distance, radius as u32);
                self.draw_pixel((x0 + x) as u32, (y0 + y) as u32, &blended_color);
            }
        }
    }

    fn move_area(&mut self, x: u32, y: u32, width: u32, height: u32, dx: i32, dy: i32) {
        let mut temp_buffer: Vec<Option<Color>> = vec![None; (width * height) as usize];

        for i in 0..height {
            for j in 0..width {
                let orig_x = x + j;
                let orig_y = y + i;
                let color = self.get_pixel(orig_x, orig_y);
                temp_buffer[(i * width + j) as usize] = Some(color);
            }
        }

        self.fill_rect(x, y, width, height, &Color::BLACK);

        let new_x = (x as i32 + dx) as u32;
        let new_y = (y as i32 + dy) as u32;

        for i in 0..height {
            for j in 0..width {
                if let Some(color) = temp_buffer[(i * width + j) as usize].as_ref() {
                    self.draw_pixel(new_x + j, new_y + i, color);
                }
            }
        }
    }

    fn move_up(&mut self, dy: u32) {
        let framebuffer = self.framebuffer.get_mut();
        let offset = (dy * self.mode.pitch) as usize;
        let size = framebuffer.len() - offset;
        framebuffer.copy_within(offset.., 0);
        framebuffer[size..].fill(0);
    }

    fn clear(&mut self) {
        let framebuffer = self.framebuffer.get_mut();
        framebuffer.fill(0)
    }
}