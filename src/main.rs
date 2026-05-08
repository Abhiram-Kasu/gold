use logos::Logos;

use crate::lexer::Token;

mod lexer;

fn main() {
    for i in Token::lexer("let item: Int = 53") {
        println!("{:?}", i);
    }
}
