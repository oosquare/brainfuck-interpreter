//! # brainfuck-interpreter
//! A fast, powerful and configurable interpreter written in Rust,
//! which allows various options to meet different demends, including
//! memory (tape) length configuration, EOF handling configuration and
//! so on.
//!
//! Licensed under MIT.
//!
//! Copyright (C) 2023 Justin Chen (ctj12461)
//!

#![allow(
    clippy::collapsible_else_if,
    clippy::new_without_default,
    clippy::comparison_chain
)]

pub mod interpreter;
mod lexer;
pub mod parser;

pub use interpreter::memory::{
    config::{self as memory_config, Config as MemoryConfig},
    MemoryError,
};
pub use interpreter::processor::ProcessorError;
pub use interpreter::stream::config::{self as stream_config, Config as StreamConfig};
pub use interpreter::{Interpreter, InterpreterError};
pub use parser::syntax::ParseError;
