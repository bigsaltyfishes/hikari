use conquer_once::spin::OnceCell;
use core::fmt;
use spin::{Mutex, MutexGuard};

use crate::common::debug::graphics::canvas::{Color, SimpleCanvas};
use crate::common::debug::graphics::screen::Screen;
use crate::devices::efifb::SCREEN_INSTANCE;

pub static CONSOLE_INSTANCE: OnceCell<Mutex<Console<'static>>> = OnceCell::uninit();

const BACKUP_CHAR: char = '�';
const CHAR_HEIGHT: usize = 16;

pub struct Console<'a> {
    screen: MutexGuard<'a, Screen<'a>>,
    line_spacing: usize,
    letter_spacing: usize,
    border_padding: usize,
    x_pos: usize,
    y_pos: usize,
}

impl<'a> Console<'a> {
    pub fn new(screen: MutexGuard<'a, Screen<'a>>) -> Self {
        Self {
            screen,
            line_spacing: 2,
            letter_spacing: 0,
            border_padding: 1,
            x_pos: 1,
            y_pos: 1,
        }
    }

    pub fn set_line_spacing(&mut self, spacing: usize) {
        self.line_spacing = spacing;
    }

    pub fn set_letter_spacing(&mut self, spacing: usize) {
        self.letter_spacing = spacing;
    }

    pub fn set_border_padding(&mut self, padding: usize) {
        self.border_padding = padding;
    }
}

impl Console<'_> {
    pub fn clear(&mut self) {
        self.screen.clear();
        self.y_pos = self.border_padding;
        self.x_pos = self.border_padding;
    }

    pub fn new_line(&mut self) {
        self.x_pos = self.border_padding;
        self.y_pos += self.line_spacing + CHAR_HEIGHT;
    }

    pub fn move_up(&mut self) {
        let screen = &mut self.screen;
        let dy = self.y_pos + self.line_spacing + CHAR_HEIGHT - screen.get_mode().height as usize;
        self.y_pos -= dy;
        screen.move_up(dy as u32);
    }

    pub fn draw_char(&mut self, c: char, color: Color) {
        if c == '\n' {
            self.new_line();
        } else {
            let glyph = unifont::get_glyph(c).unwrap_or(unifont::get_glyph(BACKUP_CHAR).unwrap());
            if self.x_pos + glyph.get_width() >= self.screen.get_mode().width as usize {
                self.new_line();
            }
            if self.y_pos + CHAR_HEIGHT + self.line_spacing > self.screen.get_mode().height as usize {
                self.move_up();
            }

            for y in 0..CHAR_HEIGHT {
                for x in 0..glyph.get_width() {
                    if glyph.get_pixel(x, y) {
                        self.screen.draw_pixel(
                            (self.x_pos + x) as u32,
                            (self.y_pos + y) as u32,
                            &color,
                        );
                    }
                }
            }
            self.x_pos += glyph.get_width() + self.letter_spacing;
        }
    }
}

impl fmt::Write for Console<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut chars = s.chars();
        let mut current_color = Color::WHITE;

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                if chars.next() == Some('[') {
                    if let (Some(color_code_one), Some(color_code_two), Some('m')) = (chars.next(), chars.next(), chars.next()) {
                        if let (Some(3), Some(code)) = (color_code_one.to_digit(10), color_code_two.to_digit(10)) {
                            current_color = Color::from_ansi_code(code as u8);
                            continue;
                        }
                    }
                }
            }
            self.draw_char(ch, current_color);
        }
        Ok(())
    }
}

pub fn module_init() {
    let screen = SCREEN_INSTANCE.get().unwrap();
    let console = Console::new(screen.lock());
    CONSOLE_INSTANCE.init_once(|| Mutex::new(console));
}