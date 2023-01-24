#![allow(unused)]

pub mod strategy;

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::rc::Rc;
use strategy::{AddrRange, AddrStrategy, CellStrategy, EofStrategy, OverflowStrategy};

pub type Result<T> = std::result::Result<T, MemoryError>;

#[derive(Debug, PartialEq, Eq)]
pub enum MemoryError {
    OutOfBounds {
        now_position: isize,
        offset: isize,
        range: AddrRange,
    },
    Overflow {
        before: i32,
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
                range: AddrRange { left, right },
            } => write!(
                f,
                "MemoryError::OutOfBounds: failed to seek to address {} + {} out of [{}, {}]",
                now_position, offset, left, right
            ),
            Self::Overflow { before, add } => write!(
                f,
                "MemoryError::Overflow: {} + {} will overflow",
                before, add
            ),
        }
    }
}

pub struct Memory {
    memory: Vec<i32>,
    cur: isize,
    addr_strategy: Box<dyn AddrStrategy>,
    cell_strategy: Box<dyn CellStrategy>,
    eof_strategy: Box<dyn EofStrategy>,
    overflow_strategy: Box<dyn OverflowStrategy>,
}

impl Memory {
    pub fn new(
        addr_strategy: Box<dyn AddrStrategy>,
        cell_strategy: Box<dyn CellStrategy>,
        eof_strategy: Box<dyn EofStrategy>,
        overflow_strategy: Box<dyn OverflowStrategy>,
    ) -> Self {
        let memory = vec![0; addr_strategy.range().len()];
        let cur = addr_strategy.initial();
        Self {
            memory,
            cur,
            addr_strategy,
            cell_strategy,
            eof_strategy,
            overflow_strategy,
        }
    }

    pub fn seek(&mut self, offset: isize) -> Result<()> {
        self.cur = self.addr_strategy.seek(self.cur, offset)?;
        Ok(())
    }

    pub fn position(&self) -> isize {
        self.cur
    }

    pub fn add(&mut self, add: i32) -> Result<()> {
        let addr = self.addr_strategy.calc(self.cur);
        let target = self.memory.get_mut(addr).unwrap();
        let strategy = self.cell_strategy.as_ref();
        let res = self.overflow_strategy.add(strategy, *target, add)?;
        *target = res;
        Ok(())
    }

    pub fn set(&mut self, val: i8) {
        let addr = self.addr_strategy.calc(self.cur);
        let target = self.memory.get_mut(addr).unwrap();

        if let Some(res) = self.eof_strategy.check(val) {
            *target = res as i32;
        }
    }

    pub fn get(&self) -> i32 {
        self.memory[self.addr_strategy.calc(self.cur)]
    }
}
