#![allow(unused)]

pub mod context;
pub mod memory;
pub mod processor;
pub mod stream;

use snafu::prelude::*;

use crate::compiler::{Compiler, InstructionList, ParseError};
use context::Context;
use memory::{config::Config as MemoryConfig, Memory};
use processor::{Processor, ProcessorError};
use stream::config::Config as StreamConfig;

type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter {
    context: Context,
    processor: Option<Processor>,
}

impl Interpreter {
    pub fn new(memory_config: MemoryConfig, stream_config: StreamConfig) -> Self {
        Self {
            context: Context::new(memory_config, stream_config),
            processor: None,
        }
    }

    pub fn load(&mut self, code: &str) -> Result<()> {
        let compiler = Compiler::new();
        let instructions = compiler.compile(code)?;

        self.processor = Some(Processor::new(instructions));
        Ok(())
    }

    pub fn reset(&mut self) {
        self.processor = None;
    }

    pub fn run(&mut self) -> Result<()> {
        if let Some(processor) = self.processor.as_mut() {
            processor.run(&mut self.context)?;
            Ok(())
        } else {
            Err(InterpreterError::Uninitialized)
        }
    }
}

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum InterpreterError {
    #[snafu(display("couldn't parse the code\ncaused by: {source}"))]
    Parse { source: ParseError },
    #[snafu(display("an error occurred when running the code\ncaused by: {source}"))]
    Runtime { source: ProcessorError },
    #[snafu(display("the program hasn't been loaded yet"))]
    Uninitialized,
}

impl From<ParseError> for InterpreterError {
    fn from(e: ParseError) -> Self {
        Self::Parse { source: e }
    }
}

impl From<ProcessorError> for InterpreterError {
    fn from(e: ProcessorError) -> Self {
        Self::Runtime { source: e }
    }
}
