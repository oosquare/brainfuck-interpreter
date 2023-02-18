use common::compiler::{Compiler, ParseError};
use common::execution::context::Context;
use common::execution::memory::config::Config as MemoryConfig;
use common::execution::processor::{Processor, ProcessorError};
use common::execution::stream::config::Config as StreamConfig;

use snafu::prelude::*;

type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter {
    context: Context,
}

impl Interpreter {
    pub fn new(memory_config: MemoryConfig, stream_config: StreamConfig) -> Self {
        Self {
            context: Context::new(memory_config, stream_config),
        }
    }

    pub fn run(&mut self, code: &str) -> Result<()> {
        let compiler = Compiler::new();
        let instructions = compiler.compile(code)?;
        let mut processor = Processor::new(instructions);
        processor.run(&mut self.context)?;
        Ok(())
    }
}

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum InterpreterError {
    #[snafu(display("couldn't parse the code"))]
    Parse { source: ParseError },
    #[snafu(display("an error occurred when running the code"))]
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
