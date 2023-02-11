mod optimizer;
mod syntax;

use crate::compiler::lexer::TokenList;
use optimizer::Optimizer;
use snafu::prelude::*;
pub use syntax::{AddUntilZeroArg, SyntaxError, SyntaxTree};

type Result<T> = std::result::Result<T, ParseError>;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, token_list: TokenList) -> Result<SyntaxTree> {
        let mut optimizer = Optimizer::new();
        optimizer.load_rules();
        let tree = SyntaxTree::build(token_list)?;
        let tree = optimizer.optimize(tree);
        Ok(tree)
    }
}

#[derive(Debug, Snafu, PartialEq, Eq)]
pub enum ParseError {
    #[snafu(display("error occurred when parsing code"))]
    Syntax { source: SyntaxError },
}

impl From<SyntaxError> for ParseError {
    fn from(e: SyntaxError) -> Self {
        Self::Syntax { source: e }
    }
}
