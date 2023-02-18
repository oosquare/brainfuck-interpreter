pub mod position;
pub mod set;
pub mod view;

use snafu::prelude::*;

pub type Result<T> = std::result::Result<T, CommandError>;

#[derive(Snafu, Debug)]
pub enum CommandError {
    #[snafu(display("error occurred when executing command `view`"))]
    View { source: view::ViewError },
    #[snafu(display("error occurred when executing command `set`"))]
    Set { source: set::SetError },
}
