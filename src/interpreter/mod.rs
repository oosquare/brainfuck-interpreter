#![allow(unused)]

mod instruction;
pub mod memory;
pub mod processor;
pub mod stream;

use std::error::Error;
use std::fmt::{self, Display, Formatter};

use crate::parser::{parse, syntax::ParseError};

use instruction::InstructionList;
use memory::Memory;
use processor::{Processor, ProcessorState};
use processor::ProcessorError;
use stream::{CharStandardOutStream, StandardInStream};

type Result<T> = std::result::Result<T, InterpreterError>;

pub struct Interpreter {
    memory_config: memory::config::Config,
    stream_config: stream::config::Config,
    processor: Option<Processor>,
}

impl Interpreter {
    pub fn new(
        memory_config: memory::config::Config,
        stream_config: stream::config::Config,
    ) -> Self {
        Self {
            memory_config,
            stream_config,
            processor: None,
        }
    }

    pub fn load(&mut self, code: &str) -> Result<()> {
        let syntax_tree = parse(code)?;
        let instructions = InstructionList::compile(syntax_tree);

        let memory::config::Config {
            len,
            addr,
            cell,
            overflow,
            eof,
        } = self.memory_config.clone();

        let memory = memory::Builder::new()
            .len(len)
            .addr(addr)
            .cell(cell)
            .overflow(overflow)
            .eof(eof)
            .build();

        let stream::config::Config { input, output } = self.stream_config.clone();
        let (in_stream, out_stream) = stream::Builder::new().input(input).output(output).build();

        self.processor = Some(Processor::new(instructions, memory, in_stream, out_stream));
        Ok(())
    }

    pub fn reset(&mut self) {
        self.processor = None;
    }

    pub fn run(&mut self) -> Result<()> {
        if let Some(processor) = self.processor.as_mut() {
            processor.run()?;
            Ok(())
        } else {
            Err(InterpreterError::Uninitialized)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpreterError {
    Parse(ParseError),
    Runtime(ProcessorError),
    Uninitialized,
}

impl Error for InterpreterError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Parse(e) => Some(e),
            Self::Runtime(e) => Some(e),
            _ => None,
        }
    }
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(_) => write!(f, "InterpreterError::Parse: invalid syntax"),
            Self::Runtime(_) => write!(f, "InterpreterError::Runtime: error occured when running"),
            Self::Uninitialized => write!(f, "InterpreterError::Uninitialized: the program hasn't been loaded")
        }
    }
}

impl From<ParseError> for InterpreterError {
    fn from(e: ParseError) -> Self {
        Self::Parse(e)
    }
}

impl From<ProcessorError> for InterpreterError {
    fn from(e: ProcessorError) -> Self {
        Self::Runtime(e)
    }
}