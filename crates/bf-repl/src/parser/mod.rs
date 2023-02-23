use std::str::{FromStr, SplitWhitespace};

use crate::command::Command;
use common::execution::memory::AddrRange;

use snafu::prelude::*;

type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(&self, cmd: &str) -> Result<Command> {
        let mut split = cmd.splitn(2, ' ');
        let cmd_type = split.next().unwrap();
        let args = split.next().unwrap_or("");

        match cmd_type {
            "get" => self.parse_get(args),
            "position" => self.parse_position(args),
            "add" => self.parse_add(args),
            "set" => self.parse_set(args),
            "view" => self.parse_view(args),
            "run" => Ok(Command::Run {
                code: args.to_owned(),
            }),
            "help" => Ok(Command::Help),
            "exit" => Ok(Command::Exit),
            _ => Err(ParseError::UnknownCommand),
        }
    }

    fn into_num<T: FromStr>(&self, arg: &str) -> Result<T> {
        match arg.parse::<T>() {
            Ok(num) => Ok(num),
            Err(_) => Err(ParseError::InvalidArgument),
        }
    }

    fn split_whitespace<'a>(&self, args: &'a str) -> Result<SplitWhitespace<'a>> {
        match args {
            "" => Err(ParseError::InvalidArgument),
            args => Ok(args.split_whitespace()),
        }
    }

    fn parse_get(&self, args: &str) -> Result<Command> {
        let pos = match self.split_whitespace(args)?.collect::<Vec<_>>()[..] {
            [pos] => self.into_num(pos)?,
            _ => return Err(ParseError::InvalidArgument),
        };

        Ok(Command::Get { addr: pos })
    }

    fn parse_position(&self, args: &str) -> Result<Command> {
        match args {
            "" => Ok(Command::Position),
            _ => Err(ParseError::InvalidArgument),
        }
    }

    fn parse_add(&self, args: &str) -> Result<Command> {
        let (addr, val) = match self.split_whitespace(args)?.collect::<Vec<_>>()[..] {
            [addr, val] => (self.into_num(addr)?, self.into_num(val)?),
            _ => return Err(ParseError::InvalidArgument),
        };

        Ok(Command::Add { addr, val })
    }

    fn parse_set(&self, args: &str) -> Result<Command> {
        let (addr, val) = match self.split_whitespace(args)?.collect::<Vec<_>>()[..] {
            [addr, val] => (self.into_num(addr)?, self.into_num(val)?),
            _ => return Err(ParseError::InvalidArgument),
        };

        Ok(Command::Set { addr, val })
    }

    fn parse_view(&self, args: &str) -> Result<Command> {
        let (left, right) = match self.split_whitespace(args)?.collect::<Vec<_>>()[..] {
            [left, right] => (self.into_num(left)?, self.into_num(right)?),
            _ => return Err(ParseError::InvalidArgument),
        };

        Ok(Command::View {
            range: AddrRange { left, right },
        })
    }
}

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidArgument,
    UnknownCommand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_command() {
        let actual = parse("get -2");
        let expected = Ok(Command::Get { addr: -2 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_get_command_error() {
        let actual = parse("get 1 1");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);

        let actual = parse("get");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_run_command() {
        let actual = parse("run +++--<.>,[-] --");
        let expected = Ok(Command::Run {
            code: String::from("+++--<.>,[-] --"),
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_add_command() {
        let actual = parse("add 1 1");
        let expected = Ok(Command::Add { addr: 1, val: 1 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_add_command_error() {
        let actual = parse("add");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);

        let actual = parse("add 1");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);

        let actual = parse("add 1 1 1");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_set_command() {
        let actual = parse("set 1 1");
        let expected = Ok(Command::Set { addr: 1, val: 1 });
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_set_command_error() {
        let actual = parse("set");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);

        let actual = parse("set 1");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);

        let actual = parse("set 1 1 1");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_view_command() {
        let actual = parse("view 1 2");
        let expected = Ok(Command::View {
            range: AddrRange { left: 1, right: 2 },
        });
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_view_command_error() {
        let actual = parse("view");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);

        let actual = parse("view 1");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);

        let actual = parse("view 1 1 1");
        let expected = Err(ParseError::InvalidArgument);
        assert_eq!(actual, expected);
    }

    #[test]
    fn unknown_command_error() {
        let actual = parse("unknown");
        let expected = Err(ParseError::UnknownCommand);
        assert_eq!(actual, expected);
    }

    fn parse(cmd: &str) -> Result<Command> {
        Parser::new().parse(cmd)
    }
}
