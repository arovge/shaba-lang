use crate::lexer::{Lexer, token::{Keyword, Token, Literal}};

#[test]
fn tokenizes_str() {
    let source = r#"
        "hello, world!"
    "#;
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize();

    let expected: Vec<Token> = vec![
        Literal::String("hello, world!".to_string()).into(),
        Token::EOF
    ];

    assert_eq!(tokens, expected);
}

#[test]
fn tokenizes_snippet() {
    let source = r#"
        let str = "hello, world!"
        print(str)

        let num = 1 + 1
        let isNumGreaterThanZero = num > 0
    "#;
    let mut lexer = Lexer::new(source.to_string());
    let tokens = lexer.tokenize();

    let expected: Vec<Token> = vec![
        Keyword::Let.into(),
        Token::Identifier("str".to_string()),
        Token::Equals,
        Literal::String("hello, world!".to_string()).into(),
        Token::Identifier("print".to_string()),
        Token::OpenParen,
        Token::Identifier("str".to_string()),
        Token::CloseParen,
        Keyword::Let.into(),
        Token::Identifier("num".to_string()),
        Token::Equals,
        Token::Unknown("1".to_string()),
        Token::Plus,
        Token::Unknown("1".to_string()),
        Keyword::Let.into(),
        Token::Identifier("isNumGreaterThanZero".to_string()),
        Token::Equals,
        Token::Identifier("num".to_string()),
        Token::GreaterThan,
        Token::Unknown("0".to_string()),
        Token::EOF
    ];

    assert_eq!(tokens, expected);
}
