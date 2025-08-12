use logos::Logos;
use crate::lexer::TokenType::Whitespace;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum TokenType {
    // Values
    #[regex(r"[A-Za-z_][0-9A-Za-z_]*", |lex| Some(lex.slice().to_string()))]
    Identifier(String),
    #[regex(r"[+-]?[0-9]+", |lex| lex.slice().parse::<i64>().ok())]
    IntLiteral(i64),
    #[regex(r"[+-]?[0-9]+(?:\.[0-9]*)", |lex| lex.slice().parse::<f64>().ok())]
    FloatLiteral(f64),
    #[regex(r#"'([^'\\]|\\[nrt0'\\"])'"#, |lex| lex.slice().parse::<char>().ok())]
    CharLiteral(char),
    #[regex(r#""([^"\\]|\\[nrt0'\\"])*""#, |lex| Some(lex.slice().to_string()))]
    StringLiteral(String),

    // Punctuators
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("?")]
    Question,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token(",")]
    Comma,

    // Operators
    #[token("*")]
    Mul,
    #[token("*=")]
    MulEquals,
    #[token("/")]
    Div,
    #[token("/=")]
    DivEquals,
    #[token("%")]
    Mod,
    #[token("%=")]
    ModEquals,
    #[token("+")]
    Plus,
    #[token("+=")]
    PlusEquals,
    #[token("++")]
    PlusPlus,
    #[token("-")]
    Minus,
    #[token("-=")]
    MinusEquals,
    #[token("--")]
    MinusMinus,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEquals,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEquals,
    #[token("=")]
    Equals,
    #[token("==")]
    EqualsEquals,
    #[token("!")]
    Not, // !
    #[token("!=")]
    NotEquals,
    #[token("&")]
    And,
    #[token("&=")]
    AndEquals,
    #[token("&&")]
    LogicalAnd,
    #[token("|")]
    Or,
    #[token("|=")]
    OrEquals,
    #[token("||")]
    LogicalOr,
    #[token("^")]
    Xor,
    #[token("^=")]
    XorEquals,
    #[token("<<")]
    ShiftLeft,
    #[token("<<=")]
    ShiftLeftEquals,
    #[token(">>")]
    ShiftRight,
    #[token(">>=")]
    ShiftRightEquals,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow, // ->

    // Keywords
    #[token("func")]
    Func,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("return")]
    Return,
    #[token("int")]
    Int,
    #[token("float")]
    Float,
    #[token("bool")]
    Bool,
    #[token("char")]
    Char,
    #[token("string")]
    String,
    #[token("void")]
    Void,
    #[token("true")]
    True,
    #[token("false")]
    False,

    #[regex(r"[ \t\r\n]+")]
    Whitespace,
    #[regex(r"//[^\r\n]*", logos::skip)]
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    Comment
}
#[derive(Debug, Clone)]
pub struct Loc {
    pub line: usize,
    pub column: usize,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub loc: Loc
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}
pub struct Lexer<'a> {
    source: logos::Lexer<'a, TokenType>,
    current_location: Loc,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: TokenType::lexer(source),
            current_location: Loc { line: 1, column: 1 },
        }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token_type = self.source.next()?;

        let token_str = self.source.slice();

        match token_type {
            Ok(token_type) =>{
                let result_token = Token {
                    token_type: token_type,
                    loc: self.current_location.clone()
                };
                for c in token_str.chars() {
                    if c == '\n' {
                        self.current_location.line += 1;
                        self.current_location.column = 1;
                    } else {
                        self.current_location.column += 1;
                    }
                }
                if result_token.token_type == Whitespace {
                    return self.next();
                }
                Some(result_token)
            }
            Err(error) => {
                //TODO errors
                println!("{:?}", error);
                None
            }
        }
    }
}
