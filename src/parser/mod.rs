pub mod syntax;

use crate::lexer;
use syntax::{SyntaxTree, Result};

pub fn parse(code: &str) -> Result<SyntaxTree> {
    SyntaxTree::parse(lexer::build_token_list(code))
}