use alloc::vec::Vec;
use core::fmt;

use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

use crate::common::debug::logger;
use crate::common::structs::cell::Cell;
use crate::drivers::graphics::canvas::{Canvas, Color};
use crate::drivers::graphics::screen::Screen;
use crate::drivers::graphics::SCREEN_INSTANCES;

lazy_static!(
  pub static ref CONSOLE_INSTANCES: Cell<Vec<Mutex<Console<'static>>>> = Cell::new(Vec::new());
);

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
        if self.y_pos >= self.screen.get_mode().height as usize {
            self.move_up();
        }
    }

    pub fn move_up(&mut self) {
        let screen = &mut self.screen;
        self.y_pos -= self.line_spacing + CHAR_HEIGHT;
        screen.move_up((self.line_spacing + CHAR_HEIGHT) as u32);
    }

    pub fn draw_char(&mut self, c: char, color: Color) {
        if c == '\n' {
            self.new_line();
        } else {
            let glyph = unifont::get_glyph(c).unwrap_or(unifont::get_glyph(BACKUP_CHAR).unwrap());
            if self.x_pos + glyph.get_width() >= self.screen.get_mode().width as usize {
                self.new_line();
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
            if self.y_pos + CHAR_HEIGHT + self.line_spacing >= self.screen.get_mode().height as usize {
                self.move_up();
            }
        }
    }
}

impl fmt::Write for Console<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        /*
        for c in s.chars() {
            self.draw_char(c, Color::WHITE);
        }
        Ok(())
         */
        let mut chars = s.chars();
        let mut current_color = Color::WHITE; // 默认颜色：白色

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                // 处理ANSI转义序列
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

pub fn set_global_console() {
    for screen in SCREEN_INSTANCES.get_mut() {
        let console = Console::new(screen.lock());
        CONSOLE_INSTANCES.get_mut().push(Mutex::new(console));
    }

    // set logger output
    for instance in CONSOLE_INSTANCES.get_mut() {
        logger::add_log_output(
            instance
        )
    }
}