use std::iter::zip;

use crate::lexer::{
    token::{Keyword, Literal, SourcePosition, Token, TokenKind},
    Lexer, error::LexerError,
};

#[test]
fn tokenizes_unterminated_str() {
    let source = r#"
        let person = "Bob
    "#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize().unwrap_err();

    let expected = LexerError::UnterminatedString;

    assert_eq!(result, expected);
}

#[test]
fn tokenizes_skips_whitespace_and_comments() {
    let source = r#"
        // this is a load bearing print statement
        // please do not remove or else the whole program will break
        print("domo arigato, mr. roboto")

        // SHABA.GOV/HTTPS://SHABA
        let forget = "about it"
    "#;
    let mut lexer = Lexer::new(source);
    let result: Vec<Token> = lexer.tokenize().unwrap();

    let expected = vec![
        Token::new(
            TokenKind::Identifier(String::from("print")),
            SourcePosition::new(4, 9),
            SourcePosition::new(4, 14),
        ),
        Token::new(
            TokenKind::OpenParen,
            SourcePosition::new(4, 14),
            SourcePosition::new(4, 15),
        ),
        Token::new(
            Literal::String(String::from("domo arigato, mr. roboto")).into(),
            SourcePosition::new(4, 15),
            SourcePosition::new(4, 41),
        ),
        Token::new(
            TokenKind::CloseParen,
            SourcePosition::new(4, 41),
            SourcePosition::new(4, 42),
        ),
        Token::new(
            Keyword::Let.into(),
            SourcePosition::new(7, 9),
            SourcePosition::new(7, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("forget")),
            SourcePosition::new(7, 13),
            SourcePosition::new(7, 19),
        ),
        Token::new(
            TokenKind::Equals,
            SourcePosition::new(7, 20),
            SourcePosition::new(7, 21),
        ),
        Token::new(
            Literal::String(String::from("about it")).into(),
            SourcePosition::new(7, 22),
            SourcePosition::new(7, 32),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_bool() {
    let source = r#"
        let isAustinCool = true
    "#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize().unwrap();

    let expected = vec![
        Token::new(
            Keyword::Let.into(),
            SourcePosition::new(2, 9),
            SourcePosition::new(2, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("isAustinCool")),
            SourcePosition::new(2, 13),
            SourcePosition::new(2, 25),
        ),
        Token::new(
            TokenKind::Equals,
            SourcePosition::new(2, 26),
            SourcePosition::new(2, 27),
        ),
        Token::new(
            TokenKind::Literal(Literal::Bool(true)),
            SourcePosition::new(2, 28),
            SourcePosition::new(2, 32),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_str() {
    let source = r#"
        "hello, world!"
    "#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize().unwrap();

    let expected = vec![Token::new(
        Literal::String(String::from("hello, world!")).into(),
        SourcePosition::new(2, 9),
        SourcePosition::new(2, 24),
    )];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_snippet() {
    let source = r#"
        let str = "hello, world!"
        print(str)

        let num = 1 + 1
        let isNumGreaterThanZero = num > 0
    "#;
    let mut lexer = Lexer::new(source);
    let result = lexer.tokenize().unwrap();

    let expected: Vec<Token> = vec![
        Token::new(
            Keyword::Let.into(),
            SourcePosition::new(2, 9),
            SourcePosition::new(2, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("str")),
            SourcePosition::new(2, 13),
            SourcePosition::new(2, 16),
        ),
        Token::new(
            TokenKind::Equals,
            SourcePosition::new(2, 17),
            SourcePosition::new(2, 18),
        ),
        Token::new(
            Literal::String(String::from("hello, world!")).into(),
            SourcePosition::new(2, 19),
            SourcePosition::new(2, 34),
        ),
        Token::new(
            TokenKind::Identifier(String::from("print")),
            SourcePosition::new(3, 9),
            SourcePosition::new(3, 14),
        ),
        Token::new(
            TokenKind::OpenParen,
            SourcePosition::new(3, 14),
            SourcePosition::new(3, 15),
        ),
        Token::new(
            TokenKind::Identifier(String::from("str")),
            SourcePosition::new(3, 15),
            SourcePosition::new(3, 18),
        ),
        Token::new(
            TokenKind::CloseParen,
            SourcePosition::new(3, 18),
            SourcePosition::new(3, 19),
        ),
        Token::new(
            Keyword::Let.into(),
            SourcePosition::new(5, 9),
            SourcePosition::new(5, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("num")),
            SourcePosition::new(5, 13),
            SourcePosition::new(5, 16),
        ),
        Token::new(
            TokenKind::Equals,
            SourcePosition::new(5, 17),
            SourcePosition::new(5, 18),
        ),
        Token::new(
            Literal::Integer(1).into(),
            SourcePosition::new(5, 19),
            SourcePosition::new(5, 20),
        ),
        Token::new(
            TokenKind::Plus,
            SourcePosition::new(5, 21),
            SourcePosition::new(5, 22),
        ),
        Token::new(
            Literal::Integer(1).into(),
            SourcePosition::new(5, 23),
            SourcePosition::new(5, 24),
        ),
        Token::new(
            Keyword::Let.into(),
            SourcePosition::new(6, 9),
            SourcePosition::new(6, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("isNumGreaterThanZero")),
            SourcePosition::new(6, 13),
            SourcePosition::new(6, 33),
        ),
        Token::new(
            TokenKind::Equals,
            SourcePosition::new(6, 34),
            SourcePosition::new(6, 35),
        ),
        Token::new(
            TokenKind::Identifier(String::from("num")),
            SourcePosition::new(6, 36),
            SourcePosition::new(6, 39),
        ),
        Token::new(
            TokenKind::GreaterThan,
            SourcePosition::new(6, 40),
            SourcePosition::new(6, 41),
        ),
        Token::new(
            Literal::Integer(0).into(),
            SourcePosition::new(6, 42),
            SourcePosition::new(6, 43),
        ),
    ];

    assert_tokens_eq(result, expected);
}

fn assert_tokens_eq(result: Vec<Token>, expected: Vec<Token>) {
    assert_eq!(result.len(), expected.len());

    for (actual_token, expected_token) in zip(result, expected) {
        assert_eq!(actual_token, expected_token);
    }
}
