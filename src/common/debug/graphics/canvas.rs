#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Color {
    pub const BLACK: Self = Self { red: 0, green: 0, blue: 0, alpha: 0 };
    pub const WHITE: Self = Self { red: 255, green: 255, blue: 255, alpha: 255 };
    pub const RED: Self = Self { red: 255, green: 0, blue: 0, alpha: 255 };
    pub const GREEN: Self = Self { red: 0, green: 255, blue: 0, alpha: 255 };
    pub const BLUE: Self = Self { red: 0, green: 0, blue: 255, alpha: 255 };
    pub const YELLOW: Self = Self { red: 255, green: 255, blue: 0, alpha: 255 };

    pub fn blend(&self, other: &Color, t: u32, total: u32) -> Color {
        let blend_channel = |a, b| (a as u32 * (total - t) + b as u32 * t) / total;
        Color {
            red: blend_channel(self.red, other.red) as u8,
            green: blend_channel(self.green, other.green) as u8,
            blue: blend_channel(self.blue, other.blue) as u8,
            alpha: blend_channel(self.alpha, other.alpha) as u8,
        }
    }

    pub fn from_ansi_code(code: u8) -> Color {
        match code {
            0 => Color::BLACK,
            1 => Color::RED,
            2 => Color::GREEN,
            3 => Color::YELLOW,
            4 => Color::BLUE,
            7 => Color::WHITE,
            _ => Color::WHITE,
        }
    }

    pub fn as_slice(&self) -> [u8; 4] {
        [self.red, self.green, self.blue, self.alpha]
    }
}

#[allow(unused)]
pub trait SimpleCanvas {
    fn get_pixel(&self, x: u32, y: u32) -> Color;
    fn draw_pixel(&mut self, x: u32, y: u32, color: &Color);
    fn move_up(&mut self, dy: u32);
    fn clear(&mut self);
}