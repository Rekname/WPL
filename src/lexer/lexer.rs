use super::token::{Loc, Token, TokenType};
use logos::Logos;

#[macro_export]
macro_rules! error {
    ($loc:expr, $($args:tt)*) => {{
        eprintln!("{}:{}: {}", $loc.line, $loc.column, format!($($args)*));
    }};
}

#[derive(Debug, Clone)]
pub struct Lexer<'source> {
    source: logos::Lexer<'source, TokenType>,
    next_token: Option<Result<TokenType, ()>>,
    pub current_location: Loc,
}

impl<'source> Lexer<'source> {
    pub fn new(source: &'source str) -> Self {
        let mut lexer = TokenType::lexer(source);
        let next = None;
        Self {
            source: lexer,
            next_token: next,
            current_location: Loc { line: 1, column: 1 },
        }
    }
    pub fn peek(&self) -> Option<TokenType> {
        let mut fork = self.source.clone();
        loop {
            match fork.next()? {
                Ok(t) if t != TokenType::Whitespace => return Some(t),
                Ok(_) => continue,
                Err(_) => return None,
            }
        }
    }
}

impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token_type = self.source.next()?;

        let token_str = self.source.slice();


        match token_type {
            Ok(token_type) => {
                let result_token = Token {
                    token_type: token_type,
                    loc: self.current_location.clone(),
                };
                for c in token_str.chars() {
                    if c == '\n' {
                        self.current_location.line += 1;
                        self.current_location.column = 1;
                    } else {
                        self.current_location.column += 1;
                    }
                }
                if result_token.token_type == TokenType::Whitespace {
                    return self.next();
                }
                Some(result_token)
            }
            Err(_) => {
                //TODO errors
                error!(self.current_location, "LEXER ERROR {}", token_type.unwrap());
                None
            }
        }
    }
}
