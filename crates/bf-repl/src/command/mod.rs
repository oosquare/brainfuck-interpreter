pub mod add;
pub mod get;
pub mod help;
pub mod position;
pub mod run;
pub mod set;
pub mod view;

use common::execution::memory::AddrRange;
use snafu::prelude::*;

use crate::interpreter::Interpreter;

use self::{add::AddError, get::GetError, run::RunError, set::SetError, view::ViewError};

pub type Result<T> = std::result::Result<T, CommandError>;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Get { addr: isize },
    Position,
    Run { code: String },
    Add { addr: isize, val: i32 },
    Set { addr: isize, val: i32 },
    View { range: AddrRange },
    Help,
    Exit,
}

impl Command {
    pub fn execute(self, interpreter: &mut Interpreter) -> Result<()> {
        match self {
            Command::Get { addr } => println!("{}", get::execute(interpreter.memory(), addr)?),
            Command::Position => println!("{}", position::execute(interpreter.memory())),
            Command::Run { code } => {
                run::execute(interpreter, &code)?;
                println!();
            }
            Command::Add { addr, val } => add::execute(interpreter.memory_mut(), addr, val)?,
            Command::Set { addr, val } => set::execute(interpreter.memory_mut(), addr, val)?,
            Command::View { range } => println!("{}", view::execute(interpreter.memory(), range)?),
            Command::Help => help::execute(),
            _ => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Snafu, Debug)]
pub enum CommandError {
    #[snafu(display("an error occurred when executing command `get`"))]
    Get { source: GetError },
    #[snafu(display("an error occurred when executing command `run`"))]
    Run { source: RunError },
    #[snafu(display("an error occurred when executing command `add`"))]
    Add { source: AddError },
    #[snafu(display("an error occurred when executing command `set`"))]
    Set { source: SetError },
    #[snafu(display("an error occurred when executing command `view`"))]
    View { source: ViewError },
}

impl From<GetError> for CommandError {
    fn from(source: GetError) -> Self {
        Self::Get { source }
    }
}

impl From<RunError> for CommandError {
    fn from(source: RunError) -> Self {
        Self::Run { source }
    }
}

impl From<AddError> for CommandError {
    fn from(source: AddError) -> Self {
        Self::Add { source }
    }
}

impl From<SetError> for CommandError {
    fn from(source: SetError) -> Self {
        Self::Set { source }
    }
}

impl From<ViewError> for CommandError {
    fn from(source: ViewError) -> Self {
        Self::View { source }
    }
}
