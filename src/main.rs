pub mod parser;
pub mod lexer;

use wpl::parser::parser::Parser;

fn main() {
    let mut parser = Parser::new("if else");
    print!("{:?}",parser.parse_expression());
}
