#[cfg(test)]
mod tests {
    use std::iter::zip;
    use wpl::lexer::{Lexer, Token, TokenType, Loc};
    const DEFAULT_LOC: Loc = Loc { line: 0, column: 0 };

    #[test]
    fn identifier_test() {
        let lex: Lexer = Lexer::new(r#"foo poo _bug __x123__"#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::Identifier("foo".to_string()),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Identifier("poo".to_string()),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Identifier("_bug".to_string()),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Identifier("__x123__".to_string()),
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn string_char_literal_test() {
        let lex: Lexer = Lexer::new(r#"'a' "hello" '\n' "\t\r""#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::CharLiteral('a'),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::StringLiteral("\"hello\"".to_string()),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::CharLiteral('\n'),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::StringLiteral("\"\\t\\r\"".to_string()),
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn punctuators_test() {
        let lex: Lexer = Lexer::new(r#"(){}[]?:;,"#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::LeftParen,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::RightParen,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::LeftBrace,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::RightBrace,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::LeftBracket,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::RightBracket,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Question,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Colon,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Semicolon,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Comma,
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn operators_test() {
        let lex: Lexer = Lexer::new(r#" + - * / % = << >> == != && ||"#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::Plus,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Minus,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Mul,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Div,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Mod,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Equals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::ShiftLeft,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::ShiftRight,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::EqualsEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::NotEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::LogicalAnd,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::LogicalOr,
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn compound_operators_test() {
        let lex: Lexer = Lexer::new(r#"+= -= *= /= %= &= |= ^= <<= >>="#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::PlusEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::MinusEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::MulEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::DivEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::ModEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::AndEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::OrEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::XorEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::ShiftLeftEquals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::ShiftRightEquals,
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn keywords_test() {
        let lex: Lexer = Lexer::new(r#"func if else for while break continue return"#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::Func,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::If,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Else,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::For,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::While,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Break,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Continue,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Return,
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn type_keywords_test() {
        let lex: Lexer = Lexer::new(r#"int float bool char string void"#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::Int,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Float,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Bool,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Char,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::String,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Void,
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn boolean_literals_test() {
        let lex: Lexer = Lexer::new(r#"true false"#);
        let expected_tokens = vec![
            Token {
                token_type: TokenType::True,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::False,
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }

    #[test]
    fn comments_skipping_test() {
        let lex: Lexer = Lexer::new(
            r#"
        // This is a comment
        x = 5; /* Another comment */
        y = 10;"#,
        );
        let expected_tokens = vec![
            Token {
                token_type: TokenType::Identifier("x".to_string()),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Equals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::IntLiteral(5),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Semicolon,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Identifier("y".to_string()),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Equals,
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::IntLiteral(10),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::Semicolon,
                loc: DEFAULT_LOC,
            },
        ];
        for (expected, observed) in zip(expected_tokens.iter(), lex) {
            assert_eq!(expected, &observed);
        }
    }
    #[test]
    fn location_counting_test() {
        let lex = Lexer::new(
            r#"line1
    line2
        line3
    // comment
    x = 5 + (y * 10);"#,
        );
        let expected = vec![
            Token {
                token_type: TokenType::Identifier("line1".to_string()),
                loc: Loc { line: 1, column: 1 },
            },
            Token {
                token_type: TokenType::Identifier("line2".to_string()),
                loc: Loc { line: 2, column: 5 },
            },
            Token {
                token_type: TokenType::Identifier("line3".to_string()),
                loc: Loc { line: 3, column: 9 },
            },
            Token {
                token_type: TokenType::Identifier("x".to_string()),
                loc: Loc { line: 5, column: 5 },
            },
            Token {
                token_type: TokenType::Equals,
                loc: Loc { line: 5, column: 7 },
            },
            Token {
                token_type: TokenType::IntLiteral(5),
                loc: Loc { line: 5, column: 9 },
            },
            Token {
                token_type: TokenType::Plus,
                loc: Loc {
                    line: 5,
                    column: 11,
                },
            },
            Token {
                token_type: TokenType::LeftParen,
                loc: Loc {
                    line: 5,
                    column: 13,
                },
            },
            Token {
                token_type: TokenType::Identifier("y".to_string()),
                loc: Loc {
                    line: 5,
                    column: 14,
                },
            },
            Token {
                token_type: TokenType::Mul,
                loc: Loc {
                    line: 5,
                    column: 16,
                },
            },
            Token {
                token_type: TokenType::IntLiteral(10),
                loc: Loc {
                    line: 5,
                    column: 18,
                },
            },
            Token {
                token_type: TokenType::RightParen,
                loc: Loc {
                    line: 5,
                    column: 20,
                },
            },
            Token {
                token_type: TokenType::Semicolon,
                loc: Loc {
                    line: 5,
                    column: 21,
                },
            },
        ];

        for (expected, observed) in zip(expected, lex) {
            assert_eq!(expected, observed);
            assert_eq!(expected.loc.line, observed.loc.line);
            assert_eq!(expected.loc.column, observed.loc.column);
        }
    }
    #[test]
    fn _test() {
        let input = r#"123 45.67 -89 -12.34"#;

        let lex = Lexer::new(input);

        let expected = vec![
            Token {
                token_type: TokenType::IntLiteral(123),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::FloatLiteral(45.67),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::IntLiteral(-89),
                loc: DEFAULT_LOC,
            },
            Token {
                token_type: TokenType::FloatLiteral(-12.34),
                loc: DEFAULT_LOC,
            },
        ];

        for (expected, observed) in zip(expected, lex) {
            assert_eq!(expected, observed);
        }
    }
}
