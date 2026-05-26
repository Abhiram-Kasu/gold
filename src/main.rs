use logos::Logos;

use crate::lexer::Token;

mod lexer;
mod parser;
fn main() {
    for i in Token::lexer("let item: Int = 53") {
        println!("{:?}", i);
    }
}
