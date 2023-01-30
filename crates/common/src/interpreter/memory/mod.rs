pub mod config;
pub(super) mod strategy;

use config::{Addr, Cell, Eof, Overflow};
use snafu::prelude::*;
use strategy::{AddrRange, AddrStrategy, CellStrategy, EofStrategy, OverflowStrategy};

pub type Result<T> = std::result::Result<T, MemoryError>;

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum MemoryError {
    #[snafu(display("try to seek pointer from {} to {}, which is out of [{}, {}]",
    now_position, now_position + offset, range.left, range.right))]
    OutOfBounds {
        now_position: isize,
        offset: isize,
        range: AddrRange,
    },
    #[snafu(display("{before} + {add} will overflow"))]
    Overflow { before: i32, add: i32 },
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

pub struct Builder {
    len: usize,
    addr: Addr,
    cell: Cell,
    overflow: Overflow,
    eof: Eof,
}

impl Builder {
    pub fn new() -> Self {
        const DEFAULT_LEN: usize = 32768;
        Self {
            len: DEFAULT_LEN,
            addr: Addr::Unsigned,
            cell: Cell::I8,
            overflow: Overflow::Wrap,
            eof: Eof::Ignore,
        }
    }

    pub fn len(mut self, len: usize) -> Self {
        self.len = len;
        self
    }

    pub fn addr(mut self, addr: Addr) -> Self {
        self.addr = addr;
        self
    }

    pub fn cell(mut self, cell: Cell) -> Self {
        self.cell = cell;
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = overflow;
        self
    }

    pub fn eof(mut self, eof: Eof) -> Self {
        self.eof = eof;
        self
    }

    pub fn build(self) -> Memory {
        let addr_strategy: Box<dyn AddrStrategy> = match self.addr {
            Addr::Unsigned => Box::new(strategy::UnsignedAddrStrategy::new(self.len)),
            Addr::Signed => Box::new(strategy::SignedAddrStrategy::new((self.len + 1) / 2)),
        };
        let cell_strategy: Box<dyn CellStrategy> = match self.cell {
            Cell::I8 => Box::new(strategy::I8CellStrategy {}),
            Cell::I32 => Box::new(strategy::I32CellStrategy {}),
        };
        let overflow_strategy: Box<dyn OverflowStrategy> = match self.overflow {
            Overflow::Error => Box::new(strategy::ErrorOverflowStrategy {}),
            Overflow::Wrap => Box::new(strategy::WrapOverflowStrategy {}),
        };
        let eof_strategy: Box<dyn EofStrategy> = match self.eof {
            Eof::Zero => Box::new(strategy::ZeroEofStrategy {}),
            Eof::Keep => Box::new(strategy::KeepEofStrategy {}),
            Eof::Ignore => Box::new(strategy::IgnoreEofStrategy {}),
        };
        Memory::new(
            addr_strategy,
            cell_strategy,
            eof_strategy,
            overflow_strategy,
        )
    }
}
