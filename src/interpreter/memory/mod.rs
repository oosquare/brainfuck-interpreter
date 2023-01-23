#![allow(unused)]

use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, MemoryError>;

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    OutOfBounds {
        now_position: isize,
        offset: isize,
        half_len: isize,
    },
    Overflow {
        before_val: u8,
        add: i32,
    },
}

impl Error for MemoryError {}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OutOfBounds {
                now_position,
                offset,
                half_len,
            } => write!(
                f,
                "MemoryError::OutOfBounds: failed to seek to address {} + {} out of [{}, {}]",
                now_position,
                offset,
                -half_len,
                half_len - 1
            ),
            Self::Overflow { before_val, add } => write!(
                f,
                "MemoryError::Overflow: {} + {} is greater than u8::MAX or less than u8::MIN",
                before_val, add
            ),
        }
    }
}

pub trait Memory {
    /// Seek to `self.cur + offset` or do nothing and return `MemoryError::OutOfBounds`
    /// if `self.cur + offset` is out of bounds.
    fn seek(&mut self, offset: isize) -> Result<()>;

    fn position(&self) -> isize;

    fn add(&mut self, val: i32) -> Result<()>;

    fn get(&self) -> u8;
}

struct FixedSizedMemory {
    cur: usize,
    memory: Vec<u8>,
}

impl FixedSizedMemory {
    pub fn new(half_len: usize) -> FixedSizedMemory {
        FixedSizedMemory {
            cur: half_len,
            memory: vec![0; 2 * half_len],
        }
    }
}

impl Memory for FixedSizedMemory {
    fn seek(&mut self, offset: isize) -> Result<()> {
        let res = self.cur as isize + offset;

        if res < 0 || res as usize >= self.memory.len() {
            let half_len = self.memory.len() as isize / 2;
            Err(MemoryError::OutOfBounds {
                now_position: self.cur as isize - half_len,
                offset,
                half_len,
            })
        } else {
            self.cur = res as usize;
            Ok(())
        }
    }

    fn position(&self) -> isize {
        self.cur as isize - self.memory.len() as isize / 2
    }

    fn add(&mut self, val: i32) -> Result<()> {
        use std::primitive::u8;
        let before = self.memory[self.cur];
        let res = before as i64 + val as i64;

        if res < u8::MIN as i64 || res > u8::MAX as i64 {
            Err(MemoryError::Overflow { before_val: before, add: val })
        } else {
            self.memory[self.cur] = res as u8;
            Ok(())
        }
    }

    fn get(&self) -> u8 {
        self.memory[self.cur]
    }
}
