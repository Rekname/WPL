mod tests {
    use super::*;
    use wpl::lexer::{Lexer, Token, TokenType, Loc};
    use wpl::parser::parser::{Parser, };

    #[test]
    fn integer_literal() {
        let lexer = Lexer::new("42");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "42");
    }

    #[test]
    fn float_literal() {
        let lexer = Lexer::new("3.14");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "3.14");
    }

    #[test]
    fn string_literal() {
        let lexer = Lexer::new("\"hello\"");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "\"hello\"");
    }

    #[test]
    fn char_literal() {
        let lexer = Lexer::new("'a'");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "a");
    }

    #[test]
    fn bool_literal_true() {
        let lexer = Lexer::new("true");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "true");
    }

    #[test]
    fn bool_literal_false() {
        let lexer = Lexer::new("false");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "false");
    }

    #[test]
    fn identifier() {
        let lexer = Lexer::new("someVar");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "someVar");
    }

    #[test]
    fn parenthesized_expression() {
        let lexer = Lexer::new("(123)");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "(123)");
    }

    #[test]
    fn unary_minus() {
        let lexer = Lexer::new("-42");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "-42");
    }

    #[test]
    fn unary_not() {
        let lexer = Lexer::new("!true");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "!true");
    }

    #[test]
    fn unary_prefix_increment() {
        let lexer = Lexer::new("++x");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "++x");
    }

    #[test]
    fn unary_prefix_decrement() {
        let lexer = Lexer::new("--y");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "--y");
    }

    #[test]
    fn unary_prefix() {
        let lexer = Lexer::new("(--x)++");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "(--x)++");
    }

    #[test]
    fn binary_addition() {
        let lexer = Lexer::new("1 + 2");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "1 + 2");
    }

    #[test]
    fn binary_mixed_operations() {
        let lexer = Lexer::new("3 * 4 - 5");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "3 * 4 - 5");
    }

    #[test]
    fn equality_operations() {
        let lexer = Lexer::new("a == b != c");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "a == b != c");
    }

    #[test]
    fn relational_operations() {
        let lexer = Lexer::new("x < y <= z");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "x < y <= z");
    }

    #[test]
    fn logical_operations() {
        let lexer = Lexer::new("flag1 && flag2 || flag3");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "flag1 && flag2 || flag3");
    }

    #[test]
    fn operator_precedence_multiplication_before_addition() {
        let lexer = Lexer::new("1 + 2 * 3");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "1 + 2 * 3");
    }

    #[test]
    fn assignment_chain() {
        let lexer = Lexer::new("x = y += z *= 2");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "x = y += z *= 2");
    }

    #[test]
    fn function_call_no_args() {
        let lexer = Lexer::new("function()");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "function()");
    }

    #[test]
    fn function_call_with_args() {
        let lexer = Lexer::new("foo(1, 2)");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "foo(1, 2)");
    }

    #[test]
    fn function_call_multiple_args() {
        let lexer = Lexer::new("bar(a, b + c, \"test\", otherFunction(args))+foo()");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "bar(a, b + c, \"test\", otherFunction(args)) + foo()");
    }

    #[test]
    fn postfix_increment() {
        let lexer = Lexer::new("array[(index+len) % sizeof(array)]++");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "array[(index + len) % sizeof(array)]++");
    }

    #[test]
    fn parentheses_affect_precedence() {
        let lexer = Lexer::new("(1 + 2) * 3");
        let mut parser = Parser::new(lexer);
        let observed = parser.parse_expression();
        assert_eq!(observed, "(1 + 2) * 3");
    }

    #[test]
    #[should_panic]
    fn unmatched_parenthesis_error() {
        let lexer = Lexer::new(")");
        let mut parser = Parser::new(lexer);
        parser.parse_expression();
    }

    #[test]
    #[should_panic]
    fn incomplete_expression_error() {
        let lexer = Lexer::new("1 + ");
        let mut parser = Parser::new(lexer);
        parser.parse_expression();
    }

    #[test]
    #[should_panic]
    fn incomplete_function_call_error() {
        let lexer = Lexer::new("func(1, ");
        let mut parser = Parser::new(lexer);
        parser.parse_expression();
    }

    #[test]
    #[should_panic]
    fn invalid_unary_operator_error() {
        let lexer = Lexer::new("++");
        let mut parser = Parser::new(lexer);
        parser.parse_expression();
    }
}