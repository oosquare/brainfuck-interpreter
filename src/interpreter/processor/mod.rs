#![allow(unused)]

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::interpreter::instruction::{Instruction, InstructionList};
use crate::interpreter::memory::{Memory, MemoryError};
use crate::interpreter::stream::{InStream, OutStream};

type Result<T> = std::result::Result<T, ProcessorError>;

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
    fn new(
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
                Ok(())
            }
            Instruction::Output => {
                self.out_stream.write(self.memory.get());
                Ok(())
            }
            Instruction::Jump(target) => {
                todo!()
            }
            Instruction::JumpIfZero(target) => {
                todo!()
            }
            Instruction::Halt => {
                todo!()
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProcessorError {
    Memory(MemoryError),
    AlreadyHalted,
    Failed,
}

impl Error for ProcessorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Memory(e) => Some(e),
            Self::AlreadyHalted => None,
            Self::Failed => None,
        }
    }
}

impl Display for ProcessorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Memory(e) => {
                write!(f, "ProcessorError::Memory: invalid memory operation")
            }
            Self::AlreadyHalted => {
                write!(f, "ProcessorError::AlreadyHalted: all instructions finished")
            }
            Self::Failed => {
                write!(f, "ProcessorError::Failed: processor failed to run")
            }
        }
    }
}

impl From<MemoryError> for ProcessorError {
    fn from(e: MemoryError) -> Self {
        Self::Memory(e)
    }
}