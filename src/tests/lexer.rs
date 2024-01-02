use crate::lexer::{
    self,
    error::{LexerError, LexingError},
    token::{Keyword, Literal, SourceLocation, Token, TokenKind},
};
use std::iter::zip;

#[test]
fn tokenizes_unterminated_str() {
    let source = r#"
        let person = "Bob
    "#;
    let result = lexer::tokenize(source).unwrap_err();

    let expected = LexerError::new(
        LexingError::UnterminatedString,
        SourceLocation::new(2, 22),
        SourceLocation::new(3, 5),
    );

    assert_eq!(result, expected);
}

#[test]
fn tokenizes_unknown_lexme() {
    let source = r#"
        let amogus = ඞ
    "#;
    let result = lexer::tokenize(source).unwrap_err();

    let expected = LexerError::new(
        LexingError::UnknownLexme('ඞ'),
        SourceLocation::new(2, 22),
        SourceLocation::new(2, 23),
    );

    assert_eq!(result, expected);
}

#[test]
fn tokenizes_empty_str() {
    let source = r#"
        let message = ""
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            TokenKind::Keyword(Keyword::Let),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("message")),
            SourceLocation::new(2, 13),
            SourceLocation::new(2, 20),
        ),
        Token::new(
            TokenKind::Eq,
            SourceLocation::new(2, 21),
            SourceLocation::new(2, 22),
        ),
        Token::new(
            TokenKind::Literal(Literal::String(String::from(""))),
            SourceLocation::new(2, 23),
            SourceLocation::new(2, 25),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_greater_than_eq() {
    let source = r#"
        18 >= 18
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            Literal::Int(18).into(),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 11),
        ),
        Token::new(
            TokenKind::GreaterThanEq,
            SourceLocation::new(2, 12),
            SourceLocation::new(2, 14),
        ),
        Token::new(
            Literal::Int(18).into(),
            SourceLocation::new(2, 15),
            SourceLocation::new(2, 17),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_less_than_eq() {
    let source = r#"
        14 <= 18
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            Literal::Int(14).into(),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 11),
        ),
        Token::new(
            TokenKind::LessThanEq,
            SourceLocation::new(2, 12),
            SourceLocation::new(2, 14),
        ),
        Token::new(
            Literal::Int(18).into(),
            SourceLocation::new(2, 15),
            SourceLocation::new(2, 17),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_not_eq() {
    let source = r#"
        0 != 1
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            Literal::Int(0).into(),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 10),
        ),
        Token::new(
            TokenKind::NotEq,
            SourceLocation::new(2, 11),
            SourceLocation::new(2, 13),
        ),
        Token::new(
            Literal::Int(1).into(),
            SourceLocation::new(2, 14),
            SourceLocation::new(2, 15),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_eq_eq() {
    let source = r#"
        1 == 1
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            Literal::Int(1).into(),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 10),
        ),
        Token::new(
            TokenKind::EqEq,
            SourceLocation::new(2, 11),
            SourceLocation::new(2, 13),
        ),
        Token::new(
            Literal::Int(1).into(),
            SourceLocation::new(2, 14),
            SourceLocation::new(2, 15),
        ),
    ];

    assert_tokens_eq(result, expected);
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
    let result: Vec<Token> = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            TokenKind::Identifier(String::from("print")),
            SourceLocation::new(4, 9),
            SourceLocation::new(4, 14),
        ),
        Token::new(
            TokenKind::OpenParen,
            SourceLocation::new(4, 14),
            SourceLocation::new(4, 15),
        ),
        Token::new(
            Literal::String(String::from("domo arigato, mr. roboto")).into(),
            SourceLocation::new(4, 15),
            SourceLocation::new(4, 41),
        ),
        Token::new(
            TokenKind::CloseParen,
            SourceLocation::new(4, 41),
            SourceLocation::new(4, 42),
        ),
        Token::new(
            Keyword::Let.into(),
            SourceLocation::new(7, 9),
            SourceLocation::new(7, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("forget")),
            SourceLocation::new(7, 13),
            SourceLocation::new(7, 19),
        ),
        Token::new(
            TokenKind::Eq,
            SourceLocation::new(7, 20),
            SourceLocation::new(7, 21),
        ),
        Token::new(
            Literal::String(String::from("about it")).into(),
            SourceLocation::new(7, 22),
            SourceLocation::new(7, 32),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_literal_bool() {
    let source = r#"
        let isAustinCool = true
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            Keyword::Let.into(),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("isAustinCool")),
            SourceLocation::new(2, 13),
            SourceLocation::new(2, 25),
        ),
        Token::new(
            TokenKind::Eq,
            SourceLocation::new(2, 26),
            SourceLocation::new(2, 27),
        ),
        Token::new(
            TokenKind::Literal(Literal::Bool(true)),
            SourceLocation::new(2, 28),
            SourceLocation::new(2, 32),
        ),
    ];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_literal_str() {
    let source = r#"
        "hello, world!"
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![Token::new(
        Literal::String(String::from("hello, world!")).into(),
        SourceLocation::new(2, 9),
        SourceLocation::new(2, 24),
    )];

    assert_tokens_eq(result, expected);
}

#[test]
fn tokenizes_literal_integer() {
    let source = r#"
        let age = 24
    "#;
    let result = lexer::tokenize(source).unwrap();

    let expected = vec![
        Token::new(
            TokenKind::Keyword(Keyword::Let),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("age")),
            SourceLocation::new(2, 13),
            SourceLocation::new(2, 16),
        ),
        Token::new(
            TokenKind::Eq,
            SourceLocation::new(2, 17),
            SourceLocation::new(2, 18),
        ),
        Token::new(
            TokenKind::Literal(Literal::Int(24)),
            SourceLocation::new(2, 19),
            SourceLocation::new(2, 21),
        ),
    ];

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
    let result = lexer::tokenize(source).unwrap();

    let expected: Vec<Token> = vec![
        Token::new(
            Keyword::Let.into(),
            SourceLocation::new(2, 9),
            SourceLocation::new(2, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("str")),
            SourceLocation::new(2, 13),
            SourceLocation::new(2, 16),
        ),
        Token::new(
            TokenKind::Eq,
            SourceLocation::new(2, 17),
            SourceLocation::new(2, 18),
        ),
        Token::new(
            Literal::String(String::from("hello, world!")).into(),
            SourceLocation::new(2, 19),
            SourceLocation::new(2, 34),
        ),
        Token::new(
            TokenKind::Identifier(String::from("print")),
            SourceLocation::new(3, 9),
            SourceLocation::new(3, 14),
        ),
        Token::new(
            TokenKind::OpenParen,
            SourceLocation::new(3, 14),
            SourceLocation::new(3, 15),
        ),
        Token::new(
            TokenKind::Identifier(String::from("str")),
            SourceLocation::new(3, 15),
            SourceLocation::new(3, 18),
        ),
        Token::new(
            TokenKind::CloseParen,
            SourceLocation::new(3, 18),
            SourceLocation::new(3, 19),
        ),
        Token::new(
            Keyword::Let.into(),
            SourceLocation::new(5, 9),
            SourceLocation::new(5, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("num")),
            SourceLocation::new(5, 13),
            SourceLocation::new(5, 16),
        ),
        Token::new(
            TokenKind::Eq,
            SourceLocation::new(5, 17),
            SourceLocation::new(5, 18),
        ),
        Token::new(
            Literal::Int(1).into(),
            SourceLocation::new(5, 19),
            SourceLocation::new(5, 20),
        ),
        Token::new(
            TokenKind::Plus,
            SourceLocation::new(5, 21),
            SourceLocation::new(5, 22),
        ),
        Token::new(
            Literal::Int(1).into(),
            SourceLocation::new(5, 23),
            SourceLocation::new(5, 24),
        ),
        Token::new(
            Keyword::Let.into(),
            SourceLocation::new(6, 9),
            SourceLocation::new(6, 12),
        ),
        Token::new(
            TokenKind::Identifier(String::from("isNumGreaterThanZero")),
            SourceLocation::new(6, 13),
            SourceLocation::new(6, 33),
        ),
        Token::new(
            TokenKind::Eq,
            SourceLocation::new(6, 34),
            SourceLocation::new(6, 35),
        ),
        Token::new(
            TokenKind::Identifier(String::from("num")),
            SourceLocation::new(6, 36),
            SourceLocation::new(6, 39),
        ),
        Token::new(
            TokenKind::GreaterThan,
            SourceLocation::new(6, 40),
            SourceLocation::new(6, 41),
        ),
        Token::new(
            Literal::Int(0).into(),
            SourceLocation::new(6, 42),
            SourceLocation::new(6, 43),
        ),
    ];

    assert_tokens_eq(result, expected);
}

fn assert_tokens_eq(result: Vec<Token>, expected: Vec<Token>) {
    for (actual_token, expected_token) in zip(&result, &expected) {
        if actual_token != expected_token {
            panic!(
                "\n\nExpected:\n{:?}\nActual:\n{:?}\n\n",
                expected_token, actual_token
            );
        }
    }
    assert_eq!(result.len(), expected.len());
}
