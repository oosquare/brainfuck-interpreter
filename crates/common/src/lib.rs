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

pub mod compiler;
pub mod execution;

pub use compiler::ParseError;
pub use execution::memory::{
    config::{self as memory_config, Config as MemoryConfig},
    MemoryError,
};
pub use execution::processor::ProcessorError;
pub use execution::stream::config::{self as stream_config, Config as StreamConfig};
pub use execution::{Interpreter, InterpreterError};
