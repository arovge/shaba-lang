#[derive(Debug, Clone)]
pub enum Token {
    Equals,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    Space,
    Literal(Literal),
    Identifier(String),
    Keyword(Keyword),
    EOF,
    Unknown(String)
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(String),
    FloatingPoint(String),
    Bool(bool),
    String(String)
}

impl From<Literal> for Token {
    fn from(literal: Literal) -> Token {
        Token::Literal(literal)
    }
}

#[derive(Debug, Clone)]
pub enum Keyword {
    Let,
    Var,
}
