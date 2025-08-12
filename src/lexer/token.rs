use std::cmp::PartialEq;
use logos::Logos;
use std::fmt;

#[derive(Logos, Debug, Clone, PartialEq)]
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
    Comment,
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



impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Identifier(s)   => write!(f, "{}", s),
            TokenType::IntLiteral(i)   => write!(f, "{}", i),
            TokenType::FloatLiteral(n) => write!(f, "{}", n),
            TokenType::CharLiteral(c)  => write!(f, "{}", c),
            TokenType::StringLiteral(s) => write!(f, "{}", s),

            TokenType::LeftParen    => write!(f, "("),
            TokenType::RightParen   => write!(f, ")"),
            TokenType::LeftBrace    => write!(f, "{{"),
            TokenType::RightBrace   => write!(f, "}}"),
            TokenType::LeftBracket  => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Question     => write!(f, "?"),
            TokenType::Colon        => write!(f, ":"),
            TokenType::SemiColon    => write!(f, ";"),
            TokenType::Comma        => write!(f, ","),

            TokenType::Mul            => write!(f, "*"),
            TokenType::MulEquals      => write!(f, "*="),
            TokenType::Div            => write!(f, "/"),
            TokenType::DivEquals      => write!(f, "/="),
            TokenType::Mod            => write!(f, "%"),
            TokenType::ModEquals      => write!(f, "%="),
            TokenType::Plus           => write!(f, "+"),
            TokenType::PlusEquals     => write!(f, "+="),
            TokenType::PlusPlus       => write!(f, "++"),
            TokenType::Minus          => write!(f, "-"),
            TokenType::MinusEquals    => write!(f, "-="),
            TokenType::MinusMinus     => write!(f, "--"),
            TokenType::Less           => write!(f, "<"),
            TokenType::LessEquals     => write!(f, "<="),
            TokenType::Greater        => write!(f, ">"),
            TokenType::GreaterEquals  => write!(f, ">="),
            TokenType::Equals         => write!(f, "="),
            TokenType::EqualsEquals   => write!(f, "=="),
            TokenType::Not            => write!(f, "!"),
            TokenType::NotEquals      => write!(f, "!="),
            TokenType::And            => write!(f, "&"),
            TokenType::AndEquals      => write!(f, "&="),
            TokenType::LogicalAnd     => write!(f, "&&"),
            TokenType::Or             => write!(f, "|"),
            TokenType::OrEquals       => write!(f, "|="),
            TokenType::LogicalOr      => write!(f, "||"),
            TokenType::Xor            => write!(f, "^"),
            TokenType::XorEquals      => write!(f, "^="),
            TokenType::ShiftLeft      => write!(f, "<<"),
            TokenType::ShiftLeftEquals => write!(f, "<<="),
            TokenType::ShiftRight     => write!(f, ">>"),
            TokenType::ShiftRightEquals => write!(f, ">>="),
            TokenType::Dot            => write!(f, "."),
            TokenType::Arrow          => write!(f, "->"),

            TokenType::Func     => write!(f, "func"),
            TokenType::If       => write!(f, "if"),
            TokenType::Else     => write!(f, "else"),
            TokenType::For      => write!(f, "for"),
            TokenType::While    => write!(f, "while"),
            TokenType::Break    => write!(f, "break"),
            TokenType::Continue => write!(f, "continue"),
            TokenType::Return   => write!(f, "return"),
            TokenType::Int      => write!(f, "int"),
            TokenType::Float    => write!(f, "float"),
            TokenType::Bool     => write!(f, "bool"),
            TokenType::Char     => write!(f, "char"),
            TokenType::String   => write!(f, "string"),
            TokenType::Void     => write!(f, "void"),
            TokenType::True     => write!(f, "true"),
            TokenType::False    => write!(f, "false"),

            TokenType::Whitespace => write!(f, "whitespace"),
            TokenType::Comment    => write!(f, "comment"),
        }
    }
}
