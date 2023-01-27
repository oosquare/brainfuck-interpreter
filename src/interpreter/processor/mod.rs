#![allow(unused)]

use snafu::prelude::*;

use crate::interpreter::instruction::{Instruction, InstructionList};
use crate::interpreter::memory::{Memory, MemoryError};
use crate::interpreter::stream::{InStream, OutStream};

pub type Result<T> = std::result::Result<T, ProcessorError>;

struct Counter {
    val: usize,
}

impl Counter {
    fn new() -> Self {
        Self { val: 0 }
    }

    fn tick(&mut self) {
        self.val += 1;
    }

    fn jump(&mut self, target: usize) {
        self.val = target
    }

    fn get(&self) -> usize {
        self.val
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ProcessorState {
    Ready,
    Running,
    Halted,
    Failed,
}

pub struct Processor {
    counter: Counter,
    instructions: InstructionList,
    memory: Memory,
    in_stream: Box<dyn InStream>,
    out_stream: Box<dyn OutStream>,
    state: ProcessorState,
}

impl Processor {
    pub fn new(
        instructions: InstructionList,
        memory: Memory,
        in_stream: Box<dyn InStream>,
        out_stream: Box<dyn OutStream>,
    ) -> Self {
        Self {
            counter: Counter::new(),
            instructions,
            memory,
            in_stream,
            out_stream,
            state: ProcessorState::Ready,
        }
    }

    pub fn counter(&self) -> usize {
        self.counter.get()
    }

    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    pub fn state(&self) -> ProcessorState {
        self.state
    }

    fn abort(&mut self) {
        self.state = ProcessorState::Failed;
    }

    fn tick(&mut self) {
        self.counter.tick();
        self.check_halted();
    }

    fn check_halted(&mut self) {
        if self.instructions.0[self.counter.get()] == Instruction::Halt {
            self.state = ProcessorState::Halted;
        }
    }

    pub fn step(&mut self) -> Result<()> {
        match self.state {
            ProcessorState::Halted => return Err(ProcessorError::AlreadyHalted),
            ProcessorState::Failed => return Err(ProcessorError::Failed),
            _ => {}
        }

        match self.instructions.0[self.counter.get()] {
            Instruction::Add(val) => {
                if let Err(e) = self.memory.add(val) {
                    self.abort();
                    Err(e.into())
                } else {
                    self.tick();
                    Ok(())
                }
            }
            Instruction::Seek(offset) => {
                if let Err(e) = self.memory.seek(offset) {
                    self.abort();
                    Err(e.into())
                } else {
                    self.tick();
                    Ok(())
                }
            }
            Instruction::Input => {
                self.memory.set(self.in_stream.read());
                self.tick();
                Ok(())
            }
            Instruction::Output => {
                self.out_stream.write(self.memory.get());
                self.tick();
                Ok(())
            }
            Instruction::Jump(target) => {
                self.counter.jump(target);
                self.check_halted();
                Ok(())
            }
            Instruction::JumpIfZero(target) => {
                if self.memory.get() == 0 {
                    self.counter.jump(target);
                    self.check_halted();
                } else {
                    self.tick();
                }

                Ok(())
            }
            Instruction::Halt => {
                unreachable!()
            }
        }
    }

    pub fn run(&mut self) -> Result<()> {
        match self.state {
            ProcessorState::Halted => return Err(ProcessorError::AlreadyHalted),
            ProcessorState::Failed => return Err(ProcessorError::Failed),
            _ => {}
        }

        while self.state == ProcessorState::Ready || self.state == ProcessorState::Running {
            self.step()?
        }

        Ok(())
    }
}

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum ProcessorError {
    #[snafu(display("invalid memory operation occurred\ncaused by: {source}"))]
    Memory { source: MemoryError },
    #[snafu(display("all instructions have already finished"))]
    AlreadyHalted,
    #[snafu(display("couldn't continue to run due to the previous error"))]
    Failed,
}

impl From<MemoryError> for ProcessorError {
    fn from(e: MemoryError) -> Self {
        Self::Memory { source: e }
    }
}
