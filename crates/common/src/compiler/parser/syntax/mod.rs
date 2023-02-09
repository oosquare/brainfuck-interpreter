use crate::compiler::lexer::{SingleToken, Token, TokenList};
use snafu::prelude::*;

pub type Result<T> = std::result::Result<T, SyntaxError>;

#[derive(Debug, PartialEq, Eq)]
pub struct AddUntilZeroArg {
    pub offset: isize,
    pub times: i32,
}

impl AddUntilZeroArg {
    pub fn new(offset: isize, times: i32) -> Self {
        Self { offset, times }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxTree {
    Add { val: i32 },
    Seek { offset: i32 },
    Clear,
    AddUntilZero { target: Vec<AddUntilZeroArg> },
    Input,
    Output,
    Root { block: Vec<SyntaxTree> },
    Loop { block: Vec<SyntaxTree> },
}

impl SyntaxTree {
    pub fn build(token_list: TokenList) -> Result<SyntaxTree> {
        let mut current = token_list.0.into_iter();
        let mut left_bracket_count = 0;
        let block = SyntaxTree::build_impl(&mut current, &mut left_bracket_count)?;
        Ok(SyntaxTree::Root { block })
    }

    fn build_impl<I>(current: &mut I, left_bracket_count: &mut i32) -> Result<Vec<SyntaxTree>>
    where
        I: Iterator<Item = Token>,
    {
        let mut res: Vec<SyntaxTree> = vec![];

        loop {
            if let Some(Token { token, count }) = current.next() {
                match token {
                    SingleToken::Add => res.push(SyntaxTree::Add { val: count }),
                    SingleToken::GreaterThan => res.push(SyntaxTree::Seek { offset: count }),
                    SingleToken::Comma => {
                        for _ in 0..count {
                            res.push(SyntaxTree::Input)
                        }
                    }
                    SingleToken::Dot => {
                        for _ in 0..count {
                            res.push(SyntaxTree::Output)
                        }
                    }
                    SingleToken::LeftBracket => {
                        *left_bracket_count += 1;
                        let block = SyntaxTree::build_impl(current, left_bracket_count)?;
                        res.push(SyntaxTree::Loop { block })
                    }
                    SingleToken::RightBracket => {
                        *left_bracket_count -= 1;
                        ensure!(*left_bracket_count >= 0, UnpairedRightBracketSnafu);
                        break;
                    }
                    // Both `SingleToken::Sub` and `SingleToken::LessThan` have been
                    // converted to `SingleToken::Add` and `SingleToken::GreaterThan`.
                    SingleToken::Sub | SingleToken::LessThan => {}
                }
            } else {
                if *left_bracket_count == 0 {
                    break;
                } else if *left_bracket_count > 0 {
                    return Err(SyntaxError::UnpairedLeftBracket);
                }
                // It's impossible to reach where `left_bracket_count < 0`, for it has
                // been already checked above.
            }
        }

        Ok(res)
    }
}

#[derive(Snafu, Debug, PartialEq, Eq)]
pub enum SyntaxError {
    #[snafu(display("unpaired `[` was found, and expect another `]`"))]
    UnpairedLeftBracket,
    #[snafu(display("unpaired `]` was found, and expect another `[`"))]
    UnpairedRightBracket,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_syntax_tree() {
        let tokens = TokenList(vec![
            Token::new(SingleToken::Add, 1),
            Token::new(SingleToken::Dot, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::GreaterThan, -2),
            Token::new(SingleToken::Comma, 1),
            Token::new(SingleToken::GreaterThan, 1),
            Token::new(SingleToken::RightBracket, 1),
        ]);

        let expected = Ok(SyntaxTree::Root {
            block: vec![
                SyntaxTree::Add { val: 1 },
                SyntaxTree::Output,
                SyntaxTree::Loop {
                    block: vec![
                        SyntaxTree::Seek { offset: -2 },
                        SyntaxTree::Input,
                        SyntaxTree::Seek { offset: 1 },
                    ],
                },
            ],
        });

        assert_eq!(SyntaxTree::build(tokens), expected);
    }

    #[test]
    fn unpaired_left_bracket() {
        let tokens = TokenList(vec![
            Token::new(SingleToken::Add, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::LessThan, 2),
        ]);

        let expected = Err(SyntaxError::UnpairedLeftBracket);
        assert_eq!(SyntaxTree::build(tokens), expected);
    }

    #[test]
    fn unpaired_right_bracket() {
        let tokens = TokenList(vec![
            Token::new(SingleToken::Add, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::RightBracket, 1),
            Token::new(SingleToken::RightBracket, 1),
            Token::new(SingleToken::LessThan, 2),
        ]);

        let expected = Err(SyntaxError::UnpairedRightBracket);
        assert_eq!(SyntaxTree::build(tokens), expected);
    }
}
