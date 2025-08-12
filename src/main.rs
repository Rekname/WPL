mod lexer;
use lexer::*;
fn main() {
    let lex: Lexer = Lexer::new("if else 1 2.3 \n 3 \n -1.2 ");
    for token in lex {
        println!("{:?}", token);
    }
}
