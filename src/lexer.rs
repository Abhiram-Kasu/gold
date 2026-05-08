// What do we want out language to look like?
// let or var for mutable/immutable
// types are post name
//      ex: `let hello: str = "Hello World" `

// Feature:
// custom allocators for regions, if through escape analysis an item leaves a region, it will be allocated on parent allocator
// every region has allocators with std::alloc as the base allocator using malloc/free from libc
//
// functions are just lambdas. First parameter can be a ref to the original
//
//

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip Spaces
pub enum Token {
    #[token("let")]
    Let,
    #[regex("[a-zA-Z][a-zA-Z0-9]*", |ident| ident.slice().to_string())]
    Identifier(String),
    #[regex(r":[ ]*[a-zA-Z][a-zA-Z0-9]*", |lex| {
        lex.slice()
            .trim_start_matches(':')
            .trim()
            .to_string()
    })]
    Type(String),
    #[token("=")]
    Equals,

    #[regex(r"[0-9]+\.[0-9]+", |lex| {
        lex.slice().parse::<f64>().unwrap()
    })]
    FloatLiteral(f64),

    #[regex(r"[0-9]+", |lex| {
        lex.slice().parse::<i64>().unwrap()
    })]
    IntegerLiteral(i64),
    #[regex(r#""([^"\\]|\\.)*""#, |lex| {
            let slice = lex.slice();
            slice[1..slice.len()-1].to_string()
        })]
    StringLiteral(String),
    #[regex(r#"'([^"\\]|\\.){1}'"#, |lex| {
            let slice = lex.slice();
            slice[1..slice.len()-1].as_bytes()[0]
        })]
    CharLiteral(u8),
}

#[cfg(test)]
mod lexer_tests {
    use logos::Lexer;

    use super::*;

    fn parse<'a>(str: &'a str) -> Lexer<'a, Token> {
        Token::lexer(str)
    }

    #[test]
    fn testVariable() {
        let mut res = parse("let item = 5");
        assert_eq!(res.next(), Some(Ok(Token::Let)));

        assert_eq!(res.next(), Some(Ok(Token::Identifier("item".into()))));
        assert_eq!(res.next(), Some(Ok(Token::Equals)));
        assert_eq!(res.next(), Some(Ok(Token::IntegerLiteral(5))));
    }

    #[test]
    fn testVariableWithType() {
        let mut res = parse("let item: i32 = 5");
        dbg!("{:?}", &res);

        assert_eq!(res.next(), Some(Ok(Token::Let)));

        assert_eq!(res.next(), Some(Ok(Token::Identifier("item".into()))));
        assert_eq!(res.slice(), "item");

        assert_eq!(res.next(), Some(Ok(Token::Type("i32".into()))));
    }
    #[test]
    fn testStringVariable() {
        let mut res = parse("let str = \"Hello World\"");
        assert_eq!(res.next(), Some(Ok(Token::Let)));

        assert_eq!(res.next(), Some(Ok(Token::Identifier("str".into()))));
        assert_eq!(res.next(), Some(Ok(Token::Equals)));
        assert_eq!(
            res.next(),
            Some(Ok(Token::StringLiteral("Hello World".into())))
        );
    }

    #[test]
    fn testCharVariable() {
        let mut res = parse("let char = 'H'");
        assert_eq!(res.next(), Some(Ok(Token::Let)));

        assert_eq!(res.next(), Some(Ok(Token::Identifier("char".into()))));
        assert_eq!(res.next(), Some(Ok(Token::Equals)));
        assert_eq!(res.next(), Some(Ok(Token::CharLiteral('H' as u8))));
    }
}
