#![allow(unused)]

use crate::lexer::{SingleToken, Token, TokenList};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxTree {
    Add(i32),
    Seek(i32),
    Input,
    Output,
    Root(Vec<SyntaxTree>),
    Loop(Vec<SyntaxTree>),
}

impl SyntaxTree {
    pub(super) fn parse(tokens: TokenList) -> Result<SyntaxTree> {
        let mut current = tokens.0.into_iter();
        let mut left_bracket_count = 0;
        let block = SyntaxTree::parse_impl(&mut current, &mut left_bracket_count)?;
        Ok(SyntaxTree::Root(block))
    }

    fn parse_impl<I>(
        current: &mut I,
        left_bracket_count: &mut i32,
    ) -> Result<Vec<SyntaxTree>>
    where
        I: Iterator<Item = Token>,
    {
        let mut res: Vec<SyntaxTree> = vec![];

        loop {
            if let Some(Token { token, count }) = current.next() {
                match token {
                    SingleToken::Add => res.push(SyntaxTree::Add(count)),
                    SingleToken::GreaterThan => res.push(SyntaxTree::Seek(count)),
                    SingleToken::Comma => res.push(SyntaxTree::Input),
                    SingleToken::Dot => res.push(SyntaxTree::Output),
                    SingleToken::LeftBracket => {
                        *left_bracket_count += 1;
                        let block = SyntaxTree::parse_impl(current, left_bracket_count)?;
                        res.push(SyntaxTree::Loop(block))
                    }
                    SingleToken::RightBracket => {
                        *left_bracket_count -= 1;

                        if *left_bracket_count < 0 {
                            return Err(ParseError::UnpairedRightBracket);
                        }

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
                    return Err(ParseError::UnpairedLeftBracket);
                }
                // It's impossible to reach where `left_bracket_count < 0`, for it has
                // been already checked above.
            }
        }

        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    UnpairedLeftBracket,
    UnpairedRightBracket,
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnpairedLeftBracket => write!(f, "ParseError::UnpairedLeftBracket: expect `]`"),
            Self::UnpairedRightBracket => write!(f, "ParseError::UnpairedRightBracket: expect `[`"),
        }
    }
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

        let expected = Ok(SyntaxTree::Root(vec![
            SyntaxTree::Add(1),
            SyntaxTree::Output,
            SyntaxTree::Loop(vec![
                SyntaxTree::Seek(-2),
                SyntaxTree::Input,
                SyntaxTree::Seek(1),
            ]),
        ]));

        assert_eq!(SyntaxTree::parse(tokens), expected);
    }

    #[test]
    fn unpaired_left_bracket() {
        let tokens = TokenList(vec![
            Token::new(SingleToken::Add, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::LessThan, 2),
        ]);

        let expected = Err(ParseError::UnpairedLeftBracket);
        assert_eq!(SyntaxTree::parse(tokens), expected);
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

        let expected = Err(ParseError::UnpairedRightBracket);
        assert_eq!(SyntaxTree::parse(tokens), expected);
    }
}
