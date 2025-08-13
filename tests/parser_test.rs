mod tests {
    use wpl::parser::parser::{Parser, };

    #[test]
    fn integer_literal() {
        let mut parser = Parser::new("42");
        let observed = parser.parse_expression();
        assert_eq!(observed, "42");
    }

    #[test]
    fn float_literal() {
        let mut parser = Parser::new("3.14");
        let observed = parser.parse_expression();
        assert_eq!(observed, "3.14");
    }

    #[test]
    fn string_literal() {
        let mut parser = Parser::new("\"hello\"");
        let observed = parser.parse_expression();
        assert_eq!(observed, "\"hello\"");
    }

    #[test]
    fn char_literal() {
        let mut parser = Parser::new("'a'");
        let observed = parser.parse_expression();
        assert_eq!(observed, "a");
    }

    #[test]
    fn bool_literal_true() {
        let mut parser = Parser::new("true");
        let observed = parser.parse_expression();
        assert_eq!(observed, "true");
    }

    #[test]
    fn bool_literal_false() {
        let mut parser = Parser::new("false");
        let observed = parser.parse_expression();
        assert_eq!(observed, "false");
    }

    #[test]
    fn identifier() {
        let mut parser = Parser::new("someVar");
        let observed = parser.parse_expression();
        assert_eq!(observed, "someVar");
    }

    #[test]
    fn parenthesized_expression() {
        let mut parser = Parser::new("(123)");
        let observed = parser.parse_expression();
        assert_eq!(observed, "(123)");
    }

    #[test]
    fn unary_minus() {
        let mut parser = Parser::new("-42");
        let observed = parser.parse_expression();
        assert_eq!(observed, "-42");
    }

    #[test]
    fn unary_not() {
        let mut parser = Parser::new("!true");
        let observed = parser.parse_expression();
        assert_eq!(observed, "!true");
    }

    #[test]
    fn unary_prefix_increment() {
        let mut parser = Parser::new("++x");
        let observed = parser.parse_expression();
        assert_eq!(observed, "++x");
    }

    #[test]
    fn unary_prefix_decrement() {
        let mut parser = Parser::new("--y");
        let observed = parser.parse_expression();
        assert_eq!(observed, "--y");
    }

    #[test]
    fn unary_prefix() {
        let mut parser = Parser::new("(--x)++");
        let observed = parser.parse_expression();
        assert_eq!(observed, "(--x)++");
    }

    #[test]
    fn binary_addition() {
        let mut parser = Parser::new("1 + 2");
        let observed = parser.parse_expression();
        assert_eq!(observed, "1 + 2");
    }

    #[test]
    fn binary_mixed_operations() {
        let mut parser = Parser::new("3 * 4 - 5");
        let observed = parser.parse_expression();
        assert_eq!(observed, "3 * 4 - 5");
    }

    #[test]
    fn equality_operations() {
        let mut parser = Parser::new("a == b != c");
        let observed = parser.parse_expression();
        assert_eq!(observed, "a == b != c");
    }

    #[test]
    fn relational_operations() {
        let mut parser = Parser::new("x < y <= z");
        let observed = parser.parse_expression();
        assert_eq!(observed, "x < y <= z");
    }

    #[test]
    fn logical_operations() {
        let mut parser = Parser::new("flag1 && flag2 || flag3");
        let observed = parser.parse_expression();
        assert_eq!(observed, "flag1 && flag2 || flag3");
    }

    #[test]
    fn operator_precedence_multiplication_before_addition() {
        let mut parser = Parser::new("1 + 2 * 3");
        let observed = parser.parse_expression();
        assert_eq!(observed, "1 + 2 * 3");
    }

    #[test]
    fn assignment_chain() {
        let mut parser = Parser::new("x = y += z *= 2");
        let observed = parser.parse_expression();
        assert_eq!(observed, "x = y += z *= 2");
    }

    #[test]
    fn function_call_no_args() {
        let mut parser = Parser::new("function()");
        let observed = parser.parse_expression();
        assert_eq!(observed, "function()");
    }

    #[test]
    fn function_call_with_args() {
        let mut parser = Parser::new("foo(1, 2)");
        let observed = parser.parse_expression();
        assert_eq!(observed, "foo(1, 2)");
    }

    #[test]
    fn function_call_multiple_args() {
        let mut parser = Parser::new("bar(a, b + c, \"test\", otherFunction(args))+foo()");
        let observed = parser.parse_expression();
        assert_eq!(observed, "bar(a, b + c, \"test\", otherFunction(args)) + foo()");
    }

    #[test]
    fn postfix_increment() {
        let mut parser = Parser::new("array[(index+len) % sizeof(array)]++");
        let observed = parser.parse_expression();
        assert_eq!(observed, "array[(index + len) % sizeof(array)]++");
    }

    #[test]
    fn parentheses_affect_precedence() {
        let mut parser = Parser::new("(1 + 2) * 3");
        let observed = parser.parse_expression();
        assert_eq!(observed, "(1 + 2) * 3");
    }

    #[test]
    fn variable_declaration_test() {
        let mut parser = Parser::new("int x = 1;");
        let observed = parser.parse_variable_declaration();
        assert_eq!(observed, "int x = 1;");
    }

    #[test]
    fn multi_variable_declaration_test() {
        let mut parser = Parser::new("float x = 1.5,y=-2.3 ,  z = 1.0;");
        let observed = parser.parse_variable_declaration();
        assert_eq!(observed, "float x = 1.5, y = -2.3, z = 1;");
    }

    #[test]
    #[should_panic]
    fn variable_declaration_without_semicolon_test() {
        let mut parser = Parser::new("float x = 1.5,y=-2.3 ,  z = 1.0");
        let observed = parser.parse_variable_declaration();
        assert_eq!(observed, "float x = 1.5, y = -2.3, z = 1;");
    }
    #[test]
    #[should_panic]
    fn variable_declaration_without_comma_test() {
        let mut parser = Parser::new("float x = 1.5 y=2.3");
        let observed = parser.parse_variable_declaration();
        assert_eq!(observed, "float x = 1.5, y = 2.3;");
    }

    #[test]
    #[should_panic]
    fn unmatched_parenthesis_error() {
        let mut parser = Parser::new(")");
        parser.parse_expression();
    }

    #[test]
    #[should_panic]
    fn incomplete_expression_error() {
        let mut parser = Parser::new("1 + ");
        parser.parse_expression();
    }

    #[test]
    #[should_panic]
    fn incomplete_function_call_error() {
        let mut parser = Parser::new("func(1, ");
        parser.parse_expression();
    }

    #[test]
    #[should_panic]
    fn invalid_unary_operator_error() {
        let mut parser = Parser::new("++");
        parser.parse_expression();
    }

    #[test]
    fn simple_for_loop() {
        let mut parser = Parser::new("for (i = 0; i < 10; i++) {\n x += i; }");
        let observed = parser.parse_statement();
        assert_eq!(observed, "for (i = 0; i < 10; i++) { x += i;\n}");
    }
    #[test]
    fn complex_control_flow() {
        let mut parser = Parser::new(
            "for (i=0;i<10;i++) {
                if (i%2==0) {
                    even();
                } else {
                    while (x-- > 0) {
                        odd();
                    }
                }
            }"
        );
        let observed = parser.parse_statement();
        assert_eq!(
            observed,
            "for (i = 0; i < 10; i++) { if (i % 2 == 0) { even();\n} else { while (x-- > 0) { odd();\n}\n}\n}"
        );
    }

    #[test]
    fn simple_while_loop() {
        let mut parser = Parser::new("while (x < 10) { x++; }");
        let observed = parser.parse_statement();
        assert_eq!(observed, "while (x < 10) { x++;\n}");
    }
    #[test]
    fn nested_for_loops() {
        let mut parser = Parser::new("for (i=0;i<10;i++) { for (j=0;j<10;j++) { x += i*j; } }");
        let observed = parser.parse_statement();
        assert_eq!(observed, "for (i = 0; i < 10; i++) { for (j = 0; j < 10; j++) { x += i * j;\n}\n}");
    }
    #[test]
    fn while_with_complex_condition() {
        let mut parser = Parser::new("while (x < 10 && y > 0 || !done) { process(); }");
        let observed = parser.parse_statement();
        assert_eq!(observed, "while (x < 10 && y > 0 || !done) { process();\n}");
    }

    #[test]
    fn simple_if_statement() {
        let mut parser = Parser::new("if (x > 0) { do_something(x); }");
        let observed = parser.parse_statement();
        assert_eq!(observed, "if (x > 0) { do_something(x);\n} ");
    }
    #[test]
    fn if_with_else() {
        let mut parser = Parser::new("if (x > 0) { pos(); } else { neg(); }");
        let observed = parser.parse_statement();
        assert_eq!(observed, "if (x > 0) { pos();\n} else { neg();\n}");
    }
    #[test]
    fn if_else_if_chain() {
        let mut parser = Parser::new("if (x > 0) { pos(); } else if (x < 0) { neg(); } else { zero(); }");
        let observed = parser.parse_statement();
        assert_eq!(observed, "if (x > 0) { pos();\n} else if (x < 0) { neg();\n} else { zero();\n}");
    }
    #[test]
    fn function_declaration_without_parameters_test() {
        let mut parser = Parser::new("func int func_name(){}");
        let observed = parser.parse_function_declaration();
        assert_eq!(observed, "int func_name()\n{ }");
    }
    #[test]
    fn function_declaration_with_two_parameters_test() {
        let mut parser = Parser::new("func int func_name(int a,int b){}");
        let observed = parser.parse_function_declaration();
        assert_eq!(observed, "int func_name(int a, int b)\n{ }");
    }
    #[test]
    fn function_declaration_with_definition_test() {
        let mut parser = Parser::new("func int func_name(int a,int b){int x = a+b;}");
        let observed = parser.parse_function_declaration();
        assert_eq!(observed, "int func_name(int a, int b)\n{ int x = a + b;\n}");
    }
}