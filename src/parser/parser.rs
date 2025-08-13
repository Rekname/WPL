use crate::error;
use crate::lexer::Loc;
use crate::lexer::lexer::Lexer;
use crate::lexer::{Token, TokenType};
use std::collections::{HashMap, VecDeque};
use std::iter::Peekable;

pub struct Parser<'source> {
    lexer: Peekable<Lexer<'source>>,
    generated_code: String,
    symbol_tables: VecDeque<HashMap<String, String>>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        let mut parser = Self {
            lexer: Lexer::new(source).peekable(),
            generated_code: String::new(),
            symbol_tables: VecDeque::new(),
        };
        parser.begin_scope();
        parser
    }
    fn peek_token(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }
    fn next_token(&mut self) -> Option<Token> {
        self.lexer.next()
    }
    fn current_location(&mut self) -> Loc {
        match self.peek_token() {
            Some(token) => token.loc.clone(),
            None => Loc { line: 1, column: 1 },
        }
    }
    fn begin_scope(&mut self) {
        self.symbol_tables.push_back(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.symbol_tables.pop_back();
    }
    fn add_symbol(&mut self, name: String, type_: String) {
        if let Some(current_scope) = self.symbol_tables.back_mut() {
            current_scope.insert(name, type_);
        }
    }
    fn get_c_type(token: Token) -> String {
        match token.token_type {
            TokenType::Int => "int".to_string(),
            TokenType::Float => "float".to_string(),
            TokenType::Bool => "bool".to_string(),
            TokenType::Char => "char".to_string(),
            TokenType::String => "char*".to_string(),
            TokenType::Void => "void".to_string(),
            _ => {
                error!(
                    token.loc,
                    "ERROR: Unexpected token type for C type conversion"
                );
                "".to_string()
            }
        }
    }

    fn get_symbol_type(&self, name: &str) -> Option<&str> {
        for scope in self.symbol_tables.iter() {
            if let Some(type_) = scope.get(name) {
                return Some(type_);
            }
        }
        None
    }

    fn expect_token_type(&mut self, expected: TokenType) -> bool {
        if let Some(token) = self.lexer.peek() {
            token.token_type == expected
        } else {
            false
        }
    }
    fn expect_token_types(&mut self, expected: &[TokenType]) -> bool {
        if let Some(token_type) = self.lexer.peek() {
            expected.contains(&token_type.token_type)
        } else {
            false
        }
    }

    fn read_and_expect_token_types(&mut self, expected: &[TokenType]) -> Option<Token> {
        if self.expect_token_types(expected) {
            self.lexer.next()
        } else {
            None
        }
    }
    fn read_and_expect_token_type(&mut self, expected: TokenType) -> Option<Token> {
        if self.expect_token_type(expected) {
            self.lexer.next()
        } else {
            None
        }
    }
    fn skip_token(&mut self) {
        self.lexer.next();
    }
    pub fn parse_expression(&mut self) -> String {
        self.parse_expression_assignment()
    }
    fn parse_expression_assignment(&mut self) -> String {
        let left = self.parse_expression_logical_or();

        if let Some(op) = self.read_and_expect_token_types(&[
            TokenType::Equals,
            TokenType::PlusEquals,
            TokenType::MinusEquals,
            TokenType::MulEquals,
            TokenType::DivEquals,
            TokenType::ModEquals,
            TokenType::ShiftLeftEquals,
            TokenType::ShiftRightEquals,
            TokenType::AndEquals,
            TokenType::OrEquals,
        ]) {
            let right = self.parse_expression_assignment();
            format!("{} {} {}", left, op.token_type, right)
        } else {
            left
        }
    }
    fn parse_expression_logical_or(&mut self) -> String {
        let mut left = self.parse_expression_logical_and();

        while let Some(op) = self.read_and_expect_token_type(TokenType::LogicalOr) {
            let right = self.parse_expression_logical_and();
            left = format!("{} {} {}", left, op.token_type, right);
        }

        left
    }
    fn parse_expression_logical_and(&mut self) -> String {
        let mut left = self.parse_expression_bitwise_or();

        while let Some(op) = self.read_and_expect_token_type(TokenType::LogicalAnd) {
            let right = self.parse_expression_bitwise_or();
            left = format!("{} {} {}", left, op.token_type, right);
        }

        left
    }
    fn parse_expression_bitwise_or(&mut self) -> String {
        let mut left = self.parse_expression_bitwise_and();

        while let Some(op) = self.read_and_expect_token_type(TokenType::Or) {
            let right = self.parse_expression_bitwise_and();
            left = format!("{} {} {}", left, op.token_type, right);
        }

        left
    }
    fn parse_expression_bitwise_and(&mut self) -> String {
        let mut left = self.parse_expression_equality();

        while let Some(op) = self.read_and_expect_token_type(TokenType::And) {
            let right = self.parse_expression_equality();
            left = format!("{} {} {}", left, op.token_type, right);
        }

        left
    }
    fn parse_expression_equality(&mut self) -> String {
        let mut left = self.parse_expression_relational();

        loop {
            if let Some(op) =
                self.read_and_expect_token_types(&[TokenType::EqualsEquals, TokenType::NotEquals])
            {
                let right = self.parse_expression_relational();
                left = format!("{} {} {}", left, op.token_type, right);
            } else {
                break;
            }
        }

        left
    }

    fn parse_expression_relational(&mut self) -> String {
        let mut left = self.parse_expression_shift();

        loop {
            if let Some(op) = self.read_and_expect_token_types(&[
                TokenType::Less,
                TokenType::LessEquals,
                TokenType::Greater,
                TokenType::GreaterEquals,
            ]) {
                let right = self.parse_expression_shift();
                left = format!("{} {} {}", left, op.token_type, right);
            } else {
                break;
            }
        }

        left
    }
    fn parse_expression_shift(&mut self) -> String {
        let mut left = self.parse_expression_additive();

        loop {
            if let Some(op) =
                self.read_and_expect_token_types(&[TokenType::ShiftLeft, TokenType::ShiftRight])
            {
                let right = self.parse_expression_additive();
                left = format!("{} {} {}", left, op.token_type, right);
            } else {
                break;
            }
        }

        left
    }
    fn parse_expression_additive(&mut self) -> String {
        let mut left = self.parse_expression_multiplicative();

        loop {
            if let Some(op) = self.read_and_expect_token_types(&[TokenType::Plus, TokenType::Minus])
            {
                let right = self.parse_expression_multiplicative();
                left = format!("{} {} {}", left, op.token_type, right);
            } else {
                break;
            }
        }

        left
    }
    fn parse_expression_multiplicative(&mut self) -> String {
        let mut left = self.parse_expression_unary();

        loop {
            if let Some(op) =
                self.read_and_expect_token_types(&[TokenType::Mul, TokenType::Div, TokenType::Mod])
            {
                let right = self.parse_expression_unary();
                left = format!("{} {} {}", left, op.token_type, right);
            } else {
                break;
            }
        }

        left
    }
    fn parse_expression_unary(&mut self) -> String {
        if let Some(op) = self.read_and_expect_token_types(&[
            TokenType::PlusPlus,
            TokenType::MinusMinus,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Not,
        ]) {
            let operand = self.parse_expression_unary();
            if matches!(op.token_type, TokenType::PlusPlus | TokenType::MinusMinus) {
                // TODO: lvalue check
                // if !is_lvalue(operand) {
                //     error("Prefix operator requires lvalue");
                // }
            }
            format!("{}{}", op.token_type, operand)
        } else {
            self.parse_expression_postfix()
        }
    }
    fn parse_expression_postfix(&mut self) -> String {
        let mut expr = self.parse_expression_primary();

        loop {
            if self.expect_token_type(TokenType::LeftParen) {
                self.skip_token(); // LeftParen
                let mut args = String::new();

                if !self.expect_token_type(TokenType::RightParen) {
                    args = self.parse_expression();
                    while let Some(_) = self.read_and_expect_token_type(TokenType::Comma) {
                        args.push_str(", ");
                        args.push_str(&self.parse_expression());
                    }
                }

                if !self.expect_token_type(TokenType::RightParen) {
                    error!(
                        self.current_location(),
                        "Expected ')' after function arguments"
                    );
                    panic!();
                }
                self.skip_token(); // RightParen
                expr = format!("{}({})", expr, args);
            } else if let Some(_) = self.read_and_expect_token_type(TokenType::LeftBracket) {
                // [] for arrays
                let index = self.parse_expression();
                if !self.expect_token_type(TokenType::RightBracket) {
                    error!(
                        self.current_location(),
                        "Expected ']' after array index operation"
                    );
                }
                self.skip_token(); // RightBracket
                expr = format!("{}[{}]", expr, index);
            } else if let Some(op) =
                self.read_and_expect_token_types(&[TokenType::PlusPlus, TokenType::MinusMinus])
            {
                expr.push_str(&op.token_type.to_string());
            } else {
                break;
            }
        }

        expr
    }
    fn parse_expression_primary(&mut self) -> String {
        if self.expect_token_type(TokenType::LeftParen) {
            self.skip_token(); // LeftParen
            let expr = self.parse_expression();
            self.skip_token(); // RightParen
            return format!("({})", expr);
        }

        if let Some(token) = self.lexer.next() {
            match token.token_type {
                TokenType::IntLiteral(_)
                | TokenType::FloatLiteral(_)
                | TokenType::StringLiteral(_)
                | TokenType::CharLiteral(_)
                | TokenType::True
                | TokenType::False => token.token_type.to_string(),

                TokenType::Identifier(_) => {
                    // TODO: check if declared
                    token.token_type.to_string()
                }

                _ => {
                    error!(
                        self.current_location(),
                        "Unexpected token in expression {}", token.token_type
                    );
                    panic!();
                }
            }
        } else {
            error!(
                self.current_location(),
                "Unexpected end of input in expression"
            );
            panic!();
        }
    }
}
