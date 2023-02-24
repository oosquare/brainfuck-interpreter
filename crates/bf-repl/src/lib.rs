mod command;
mod interpreter;
mod parser;

use std::error::Error;
use std::io::{self, BufRead, BufReader, Stdin, Write};

use clap::{crate_name, crate_version};
use command::Command;
use common::execution::memory::config::Config as MemoryConfig;
use common::execution::stream::config::Config as StreamConfig;
use interpreter::Interpreter;
use parser::Parser;

pub struct Repl {
    parser: Parser,
    reader: BufReader<Stdin>,
    interpreter: Interpreter,
}

impl Repl {
    const PROMPT: &str = ">>>";

    pub fn new(memory_config: MemoryConfig, stream_config: StreamConfig) -> Self {
        Self {
            parser: Parser::new(),
            reader: BufReader::new(io::stdin()),
            interpreter: Interpreter::new(memory_config, stream_config),
        }
    }

    pub fn run(&mut self) {
        let mut stdout = io::stdout();
        println!("{} {}", crate_name!(), crate_version!());
        println!("Type `help` for help.");

        loop {
            println!();
            print!("{} ", Self::PROMPT);
            stdout.flush().unwrap();

            let input = match self.read() {
                Ok(input) => input,
                Err(e) => {
                    print_error(Box::new(e));
                    continue;
                }
            };

            let cmd = match self.parse(&input) {
                Ok(cmd) => cmd,
                Err(e) => {
                    print_error(Box::new(e));
                    continue;
                }
            };

            if let Command::Exit = cmd {
                break;
            }

            if let Err(e) = cmd.execute(&mut self.interpreter) {
                print_error(Box::new(e));
            }
        }
    }

    fn read(&mut self) -> io::Result<String> {
        let mut buf = String::new();
        self.reader
            .read_line(&mut buf)
            .map(move |_| buf.trim_end().to_owned())
    }

    fn parse(&self, input: &str) -> parser::Result<Command> {
        self.parser.parse(input)
    }
}

fn print_error(e: Box<dyn Error>) {
    eprintln!("error: {e}");
    let mut e = e.source();

    while let Some(source) = e {
        eprintln!("caused by: {source}");
        e = source.source();
    }
}
