#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Equals,
    Plus,
    Minus,
    Asterisk,
    Slash,
    GreaterThan,
    LessThan,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Semicolon,
    Colon,
    Period,
    QuestionMark,
    ExclaimationPoint,
    Literal(Literal),
    Identifier(String),
    Keyword(Keyword),
    EOF,
    Unknown(String)
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Var,
}

impl From<Keyword> for Token {
    fn from(keyword: Keyword) -> Token {
        Token::Keyword(keyword)
    }
}
