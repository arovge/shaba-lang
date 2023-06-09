use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    source_position: Range<SourcePosition>,
}

impl Token {
    pub fn new(kind: TokenKind, start: SourcePosition, end: SourcePosition) -> Self {
        let source_position = start..end;
        Self {
            kind,
            source_position,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourcePosition {
    line: usize,
    column: usize,
}

impl SourcePosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
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
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i32),
    FloatingPoint(f32),
    Bool(bool),
    String(String),
}

impl From<Literal> for TokenKind {
    fn from(literal: Literal) -> TokenKind {
        TokenKind::Literal(literal)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Var,
}

impl From<Keyword> for TokenKind {
    fn from(keyword: Keyword) -> TokenKind {
        TokenKind::Keyword(keyword)
    }
}
