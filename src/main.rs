//! # brainfuck-interpreter
//! A fast, powerful and configurable interpreter written in Rust,
//! which allows various options to meet different demends, including
//! memory (tape) length configuration, EOF handling configuration and
//! so on.
//! 
//! Licensed under MIT.
//! Copyright (C) Justin Chen (ctj12461), 2023
//! 

#![allow(unused)]

use std::path::PathBuf;
use std::error::Error;

use brainfuck_interpreter::{
    Interpreter, MemoryConfig, StreamConfig, memory_config, stream_config
};
use clap::{builder::PathBufValueParser, command, value_parser, Arg};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = input();
    let (memory_config, stream_config, source) = parse(&matches);
    run(memory_config, stream_config, source)?;
    Ok(())
}

fn input() -> clap::ArgMatches {
    let cmd = command!();
    let cmd = cmd.arg(
        Arg::new("len")
            .long("len")
            .required(false)
            .value_parser(value_parser!(usize))
            .default_value("32768"),
    );
    let cmd = cmd.arg(
        Arg::new("addr")
            .long("addr")
            .required(false)
            .value_parser(["unsigned", "signed"])
            .default_value("unsigned"),
    );
    let cmd = cmd.arg(
        Arg::new("cell")
            .long("cell")
            .required(false)
            .value_parser(["int8", "int32"])
            .default_value("int8"),
    );
    let cmd = cmd.arg(
        Arg::new("overflow")
            .long("overflow")
            .required(false)
            .value_parser(["wrap", "error"])
            .default_value("wrap"),
    );
    let cmd = cmd.arg(
        Arg::new("eof")
            .long("eof")
            .required(false)
            .value_parser(["zero", "keep", "ignore"])
            .default_value("ignore"),
    );
    let cmd = cmd.arg(
        Arg::new("input")
            .long("input")
            .required(false)
            .value_parser(["null", "std"])
            .default_value("std"),
    );
    let cmd = cmd.arg(
        Arg::new("output")
            .long("output")
            .required(false)
            .value_parser(["char-std", "int-std"])
            .default_value("char-std"),
    );
    let cmd = cmd.arg(
        Arg::new("source")
            .required(true)
            .value_parser(PathBufValueParser::new()),
    );

    let matches = cmd.get_matches();
    matches
}

fn parse<'a>(matches: &'a clap::ArgMatches) -> (MemoryConfig, StreamConfig, &'a PathBuf) {
    let memory_config = MemoryConfig {
        len: *matches.get_one::<usize>("len").unwrap(),
        addr: match matches.get_one::<String>("addr").unwrap().as_str() {
            "unsigned" => memory_config::Addr::Unsigned,
            "signed" => memory_config::Addr::Unsigned,
            _ => unreachable!(),
        },
        cell: match matches.get_one::<String>("cell").unwrap().as_str() {
            "int8" => memory_config::Cell::I8,
            "int32" => memory_config::Cell::I32,
            _ => unreachable!(),
        },
        overflow: match matches.get_one::<String>("overflow").unwrap().as_str() {
            "wrap" => memory_config::Overflow::Wrap,
            "error" => memory_config::Overflow::Error,
            _ => unreachable!(),
        },
        eof: match matches.get_one::<String>("eof").unwrap().as_str() {
            "zero" => memory_config::Eof::Zero,
            "keep" => memory_config::Eof::Keep,
            "ignore" => memory_config::Eof::Ignore,
            _ => unreachable!(),
        },
    };

    let stream_config = StreamConfig {
        input: match matches.get_one::<String>("input").unwrap().as_str() {
            "null" => stream_config::Input::Null,
            "std" => stream_config::Input::Standard,
            _ => unreachable!(),
        },
        output: match matches.get_one::<String>("output").unwrap().as_str() {
            "char-std" => stream_config::Output::CharStandard,
            "int-std" => stream_config::Output::IntStandard,
            _ => unreachable!(),
        },
    };

    let source = matches.get_one::<PathBuf>("source").unwrap();
    (memory_config, stream_config, source)
}

fn run(
    memory_config: MemoryConfig,
    stream_config: StreamConfig,
    source: &PathBuf,
) -> Result<(), Box<dyn Error>> {
    let mut interpreter = Interpreter::new(memory_config, stream_config);
    let code = std::fs::read_to_string(source)?;
    interpreter.load(&code)?;
    interpreter.run()?;
    Ok(())
}