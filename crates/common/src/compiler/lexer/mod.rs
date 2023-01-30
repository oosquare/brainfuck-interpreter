#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingleToken {
    GreaterThan,
    LessThan,
    Add,
    Sub,
    Dot,
    Comma,
    LeftBracket,
    RightBracket,
}

type SingleTokenList = Vec<SingleToken>;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Token {
    pub token: SingleToken,
    pub count: i32,
}

impl Token {
    pub fn new(token: SingleToken, count: i32) -> Self {
        Self { token, count }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TokenList(pub Vec<Token>);

impl TokenList {
    /// Combine the same tokens (except `[` and ']') into a `Token`
    /// which contains the count of them.
    fn combine_same(tokens: SingleTokenList) -> TokenList {
        let mut res = vec![];
        let mut last = None::<SingleToken>;
        let mut now = None::<Token>;

        for token in tokens {
            if let Some(last) = last {
                if last == token
                    && token != SingleToken::LeftBracket
                    && token != SingleToken::RightBracket
                {
                    now.as_mut().unwrap().count += 1;
                } else {
                    res.push(now.take().unwrap());
                    now = Some(Token::new(token, 1));
                }
            } else {
                now = Some(Token::new(token, 1));
            }

            last = Some(token);
        }

        if let Some(now) = now.take() {
            res.push(now);
        }

        TokenList(res)
    }

    /// Combine `SingleToken::Add` and `SingleToken::Sub`. When it comes to
    /// `(SingleToken::Sub, 1)`, we turn it into `(SingleToken::Add, -1)`.
    fn combine_add_sub(self) -> TokenList {
        let mut res = vec![];
        let mut now = None::<Token>;

        for Token { token, count } in self.0 {
            if let SingleToken::Add = token {
                if now.is_none() {
                    now = Some(Token::new(SingleToken::Add, 0));
                }

                now.as_mut().unwrap().count += count;
                continue;
            } else if let SingleToken::Sub = token {
                if now.is_none() {
                    now = Some(Token::new(SingleToken::Add, 0));
                }

                now.as_mut().unwrap().count -= count;
                continue;
            }

            if let Some(now) = now.take() {
                if now.count != 0 {
                    res.push(now);
                }
            }

            res.push(Token::new(token, count));
        }

        if let Some(now) = now.take() {
            if now.count != 0 {
                res.push(now);
            }
        }

        TokenList(res)
    }

    /// Combine `SingleToken::LessThan` and `SingleToken::GreaterThan`. When it comes to
    /// `(SingleToken::LessThan, 1)`, we turn it into `(SingleToken::GreaterThan, -1)`.
    fn combine_less_greater(self) -> TokenList {
        let mut res = vec![];
        let mut now = None::<Token>;

        for Token { token, count } in self.0 {
            if let SingleToken::LessThan = token {
                if now.is_none() {
                    now = Some(Token::new(SingleToken::GreaterThan, 0));
                }

                now.as_mut().unwrap().count -= count;
                continue;
            } else if let SingleToken::GreaterThan = token {
                if now.is_none() {
                    now = Some(Token::new(SingleToken::GreaterThan, 0));
                }

                now.as_mut().unwrap().count += count;
                continue;
            }

            if let Some(now) = now.take() {
                if now.count != 0 {
                    res.push(now);
                }
            }

            res.push(Token::new(token, count));
        }

        if let Some(now) = now.take() {
            if now.count != 0 {
                res.push(now);
            }
        }

        TokenList(res)
    }
}

impl From<SingleTokenList> for TokenList {
    /// Combine the similar and adjacent tokens, such as `[Token::Add, Token::Add,
    /// Token::Sub]` to `[(Token::Add, 1)]`.
    fn from(tokens: SingleTokenList) -> TokenList {
        TokenList::combine_same(tokens)
            .combine_add_sub()
            .combine_less_greater()
    }
}

/// Split the program to some tokens and ignore what a brainfuck program doesn't
/// contain.
fn split(code: &str) -> Vec<char> {
    code.chars().fold(Vec::new(), |mut v, c| match c {
        c @ ('>' | '<' | '+' | '-' | '.' | ',' | '[' | ']') => {
            v.push(c);
            v
        }
        _ => v,
    })
}

fn token(ch: char) -> SingleToken {
    match ch {
        '>' => SingleToken::GreaterThan,
        '<' => SingleToken::LessThan,
        '+' => SingleToken::Add,
        '-' => SingleToken::Sub,
        '.' => SingleToken::Dot,
        ',' => SingleToken::Comma,
        '[' => SingleToken::LeftBracket,
        ']' => SingleToken::RightBracket,
        _ => unreachable!(),
    }
}

fn build_single_token_list(code: &str) -> SingleTokenList {
    split(code).into_iter().map(token).collect()
}

/// Build a `TokenList` from a brainfuck program.
pub fn build_token_list(code: &str) -> TokenList {
    TokenList::from(build_single_token_list(code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_code() {
        let code = "+ [>a+]>d.>-,.";
        let expected = vec!['+', '[', '>', '+', ']', '>', '.', '>', '-', ',', '.'];
        assert_eq!(split(code), expected);
    }

    #[test]
    fn get_token() {
        assert_eq!(token('>'), SingleToken::GreaterThan);
        assert_eq!(token('<'), SingleToken::LessThan);
        assert_eq!(token('+'), SingleToken::Add);
        assert_eq!(token('-'), SingleToken::Sub);
        assert_eq!(token('.'), SingleToken::Dot);
        assert_eq!(token(','), SingleToken::Comma);
        assert_eq!(token('['), SingleToken::LeftBracket);
        assert_eq!(token(']'), SingleToken::RightBracket);
    }

    #[test]
    fn single_token_list() {
        let list = vec![
            SingleToken::Add,
            SingleToken::Sub,
            SingleToken::Sub,
            SingleToken::LessThan,
            SingleToken::LessThan,
            SingleToken::Sub,
            SingleToken::Add,
            SingleToken::LessThan,
            SingleToken::LessThan,
            SingleToken::GreaterThan,
            SingleToken::LeftBracket,
            SingleToken::LeftBracket,
            SingleToken::RightBracket,
            SingleToken::RightBracket,
        ];
        let simplifed = TokenList::from(list);
        let expected = TokenList(vec![
            Token::new(SingleToken::Add, -1),
            Token::new(SingleToken::GreaterThan, -3),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::LeftBracket, 1),
            Token::new(SingleToken::RightBracket, 1),
            Token::new(SingleToken::RightBracket, 1),
        ]);
        assert_eq!(simplifed, expected);
    }

    #[test]
    fn empty_token_list() {
        let list: SingleTokenList = vec![];
        assert!(TokenList::from(list).0.is_empty());
    }
}
