use crate::command::Command;
use common::execution::memory::AddrRange;
use snafu::prelude::*;

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidArgument,
    UnknownCommand,
}

pub fn parse(cmd: &str) -> Result<Command> {
    todo!()
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
    }

    #[test]
    fn parse_run_command() {
        let actual = parse("run +++--<.>,[-] --");
        let expected = Ok(Command::Run { code: String::from("+++--<.>,[-] --") });
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
        let expected = Ok(Command::View { range: AddrRange { left: 1, right: 2 } });
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_view_command_error() {
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
}
