pub mod interpreter;
mod lexer;
pub mod parser;

pub use interpreter::memory::{MemoryError, config::*};
pub use interpreter::processor::ProcessorError;
pub use interpreter::stream::config::*;
pub use interpreter::{Interpreter, InterpreterError};
pub use parser::syntax::ParseError;
