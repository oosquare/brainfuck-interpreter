//! # brainfuck-interpreter
//! A fast, powerful and configurable interpreter written in Rust,
//! which allows various options to meet different demends, including
//! memory (tape) length configuration, EOF handling configuration and
//! so on.
//!
//! Licensed under MIT.
//! Copyright (C) 2023 Justin Chen (ctj12461)
//!

use std::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process;

use bf_exec::Interpreter;
use clap::{builder::PathBufValueParser, command, value_parser, Arg, ArgMatches};
use common::{memory_config, stream_config, MemoryConfig, StreamConfig};

fn main() {
    let matches = input();
    let (memory_config, stream_config, source) = parse(&matches);

    let code = match std::fs::read_to_string(source) {
        Ok(code) => code,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => eprintln!("error: couldn't find {}", source.display()),
                _ => eprintln!("error: couldn't open {}\ncaused by: {e}", source.display()),
            }

            process::exit(1);
        }
    };

    if let Err(e) = run(memory_config, stream_config, code) {
        println!("error: {e}");
        process::exit(1);
    }
}

fn input() -> ArgMatches {
    let cmd = command!();
    let cmd = cmd.arg(
        Arg::new("LEN")
            .long("len")
            .required(false)
            .value_parser(value_parser!(usize))
            .default_value("32768")
            .next_line_help(true)
            .help("the memory (tape) length the program will use.\n")
            .long_help("the memory (tape) length the program will use."),
    );
    let cmd = cmd.arg(
        Arg::new("ADDR")
            .long("addr")
            .required(false)
            .value_parser(["unsigned", "signed"])
            .default_value("unsigned")
            .next_line_help(true)
            .help("the address range of the memory (tape).\n")
            .long_help({
                let mut h = String::new();
                h.push_str("the address range of the memory (tape).\n");
                h.push('\n');
                h.push_str(" - unsigned: [0, len - 1]\n");
                h.push_str(" - signed: [-ceil(len / 2), ceil(len / 2) - 1]");
                h
            }),
    );
    let cmd = cmd.arg(
        Arg::new("CELL")
            .long("cell")
            .required(false)
            .value_parser(["int8", "int32"])
            .default_value("int8")
            .next_line_help(true)
            .help("the data type of one cell in the memory (tape).\n")
            .long_help("the data type of one cell in the memory (tape)."),
    );
    let cmd = cmd.arg(
        Arg::new("OVERFLOW")
            .long("overflow")
            .required(false)
            .value_parser(["wrap", "error"])
            .default_value("wrap")
            .next_line_help(true)
            .help("the operation the interpreter should do when an overflow error occurs.\n")
            .long_help({
                let mut h = String::new();
                h.push_str(
                    "the operation the interpreter should do when an overflow error occurs.\n",
                );
                h.push('\n');
                h.push_str(
                    " - wrap: automatically wrap the value in cell (e.g.: `127 + 1` => `-127`)\n",
                );
                h.push_str(" - error: throw an error and abort");
                h
            }),
    );
    let cmd = cmd.arg(
        Arg::new("EOF")
            .long("eof")
            .required(false)
            .value_parser(["zero", "keep", "ignore"])
            .default_value("ignore")
            .next_line_help(true)
            .help("the operation the interpreter should do when an `EOF` is read.\n")
            .long_help({
                let mut h = String::new();
                h.push_str("the operation the interpreter should do when an `EOF` is read.\n");
                h.push('\n');
                h.push_str(" - zero: turn `EOF` to `0`\n");
                h.push_str(" - keep: keep what the `EOF` is and return it (`EOF == -1`)\n");
                h.push_str(" - ignore: ignore this input and leave the cell unchanged");
                h
            }),
    );
    let cmd = cmd.arg(
        Arg::new("INPUT")
            .long("input")
            .required(false)
            .value_parser(["null", "std"])
            .default_value("std")
            .next_line_help(true)
            .help("the input stream type.\n")
            .long_help("the input stream type."),
    );
    let cmd = cmd.arg(
        Arg::new("OUTPUT")
            .long("output")
            .required(false)
            .value_parser(["char-std", "int-std"])
            .default_value("char-std")
            .next_line_help(true)
            .help("the output stream type.\n")
            .long_help("the output stream type."),
    );
    let cmd = cmd.arg(
        Arg::new("SOURCE")
            .required(true)
            .value_parser(PathBufValueParser::new())
            .next_line_help(true)
            .help("the path of the brainfuck program source code file.\n")
            .long_help("the path of the brainfuck program source code file."),
    );

    cmd.get_matches()
}

fn parse(matches: &ArgMatches) -> (MemoryConfig, StreamConfig, &PathBuf) {
    let memory_config = MemoryConfig {
        len: *matches.get_one::<usize>("LEN").unwrap(),
        addr: match matches.get_one::<String>("ADDR").unwrap().as_str() {
            "unsigned" => memory_config::Addr::Unsigned,
            "signed" => memory_config::Addr::Signed,
            _ => unreachable!(),
        },
        cell: match matches.get_one::<String>("CELL").unwrap().as_str() {
            "int8" => memory_config::Cell::I8,
            "int32" => memory_config::Cell::I32,
            _ => unreachable!(),
        },
        overflow: match matches.get_one::<String>("OVERFLOW").unwrap().as_str() {
            "wrap" => memory_config::Overflow::Wrap,
            "error" => memory_config::Overflow::Error,
            _ => unreachable!(),
        },
        eof: match matches.get_one::<String>("EOF").unwrap().as_str() {
            "zero" => memory_config::Eof::Zero,
            "keep" => memory_config::Eof::Keep,
            "ignore" => memory_config::Eof::Ignore,
            _ => unreachable!(),
        },
    };

    let stream_config = StreamConfig {
        input: match matches.get_one::<String>("INPUT").unwrap().as_str() {
            "null" => stream_config::Input::Null,
            "std" => stream_config::Input::Standard,
            _ => unreachable!(),
        },
        output: match matches.get_one::<String>("OUTPUT").unwrap().as_str() {
            "char-std" => stream_config::Output::CharStandard,
            "int-std" => stream_config::Output::IntStandard,
            _ => unreachable!(),
        },
    };

    let source = matches.get_one::<PathBuf>("SOURCE").unwrap();
    (memory_config, stream_config, source)
}

fn run(
    memory_config: MemoryConfig,
    stream_config: StreamConfig,
    code: String,
) -> Result<(), Box<dyn Error>> {
    let mut interpreter = Interpreter::new(memory_config, stream_config);
    interpreter.run(&code)?;
    Ok(())
}
