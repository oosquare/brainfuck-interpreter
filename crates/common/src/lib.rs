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

mod compiler;
mod execution;

pub use compiler::{Compiler, Instruction, InstructionList, ParseError};
pub use execution::memory_config::{self, Config as MemoryConfig};
pub use execution::stream_config::{self, Config as StreamConfig};
pub use execution::Context;
pub use execution::{Processor, ProcessorError};
