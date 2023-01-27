#![allow(unused)]

mod instruction;
pub mod memory;
pub mod processor;
pub mod stream;

use snafu::prelude::*;

use crate::parser::{parse, syntax::ParseError};

use instruction::InstructionList;
use memory::Memory;
use processor::ProcessorError;
use processor::{Processor, ProcessorState};
use stream::{InStream, OutStream};

type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Context {
    pub memory: Memory,
    pub in_stream: Box<dyn InStream>,
    pub out_stream: Box<dyn OutStream>,
}

impl Context {
    pub fn new(
        memory_config: memory::config::Config,
        stream_config: stream::config::Config,
    ) -> Self {
        let memory::config::Config {
            len,
            addr,
            cell,
            overflow,
            eof,
        } = memory_config.clone();

        let memory = memory::Builder::new()
            .len(len)
            .addr(addr)
            .cell(cell)
            .overflow(overflow)
            .eof(eof)
            .build();

        let stream::config::Config { input, output } = stream_config.clone();
        let (in_stream, out_stream) = stream::Builder::new().input(input).output(output).build();

        Self {
            memory,
            in_stream,
            out_stream,
        }
    }
}

pub struct Interpreter {
    context: Context,
    processor: Option<Processor>,
}

impl Interpreter {
    pub fn new(
        memory_config: memory::config::Config,
        stream_config: stream::config::Config,
    ) -> Self {
        Self {
            context: Context::new(memory_config, stream_config),
            processor: None,
        }
    }

    pub fn load(&mut self, code: &str) -> Result<()> {
        let syntax_tree = parse(code)?;
        let instructions = InstructionList::compile(syntax_tree);

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
