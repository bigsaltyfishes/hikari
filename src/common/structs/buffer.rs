use alloc::boxed::Box;
use core::fmt;

use circular_buffer::CircularBuffer;

use crate::common::structs::cell::Cell;

pub struct IOBuffer<const LENGTH: usize> {
    buffer: Cell<Box<CircularBuffer<LENGTH, u8>>>,
}

impl<const LENGTH: usize> IOBuffer<LENGTH> {
    pub fn new() -> IOBuffer<LENGTH> {
        Self {
            buffer: Cell::new(CircularBuffer::boxed())
        }
    }

    pub fn push(&self, data: u8) {
        self.buffer.get_mut().push_back(data);
    }

    pub fn pop(&self) -> Option<u8> {
        self.buffer.get_mut().pop_front()
    }

    pub fn clear(&self) {
        self.buffer.get_mut().clear()
    }

    pub fn capacity(&self) -> usize {
        self.buffer.get_mut().capacity()
    }

    pub fn as_slice(&self) -> (&[u8], &[u8]) {
        self.buffer.get_mut().as_slices()
    }
}

impl<const LENGTH: usize> fmt::Write for IOBuffer<LENGTH> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            self.push(*c);
        }
        Ok(())
    }
}