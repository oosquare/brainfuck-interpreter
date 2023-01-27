pub mod syntax;

use crate::lexer;
use syntax::{Result, SyntaxTree};

pub fn parse(code: &str) -> Result<SyntaxTree> {
    SyntaxTree::parse(lexer::build_token_list(code))
}
