use logos::Logos;

use crate::lexer::Token;
use crate::parser::Parser;
use crate::type_checker::TypeChecker;

mod lexer;
mod parser;
mod type_checker;
fn main() {
    let mut token_stream = vec![];
    for token in Token::lexer("let item: Int = 53") {
        token_stream.push(token.expect("Failed to lex on a token"));
    }

    let mut parser = Parser::new(token_stream);
    let ast = parser.parse().expect("Failed to parse");
    let mut typed_ast = TypeChecker::new(ast)
        .check()
        .expect("Failed to check types");
}
