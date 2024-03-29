use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    location: Range<SourceLocation>,
}

impl Token {
    pub fn new(kind: TokenKind, start: SourceLocation, end: SourceLocation) -> Self {
        let location = start..end;
        Self { kind, location }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    line: usize,
    column: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Eq,
    Plus,
    Minus,
    Asterisk,
    Slash,
    NotEq,
    GreaterThan,
    GreaterThanEq,
    EqEq,
    LessThan,
    LessThanEq,
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
    Negate,
    Literal(Literal),
    Identifier(String),
    Keyword(Keyword),
}

impl TokenKind {
    pub fn from_char(ch: char) -> Option<TokenKind> {
        match ch {
            '=' => Some(TokenKind::Eq),
            '+' => Some(TokenKind::Plus),
            '-' => Some(TokenKind::Minus),
            '*' => Some(TokenKind::Asterisk),
            '/' => Some(TokenKind::Slash),
            '>' => Some(TokenKind::GreaterThan),
            '<' => Some(TokenKind::LessThan),
            '{' => Some(TokenKind::OpenBrace),
            '}' => Some(TokenKind::CloseBrace),
            '(' => Some(TokenKind::OpenParen),
            ')' => Some(TokenKind::CloseParen),
            '[' => Some(TokenKind::OpenBracket),
            ']' => Some(TokenKind::CloseBracket),
            ',' => Some(TokenKind::Comma),
            ';' => Some(TokenKind::Semicolon),
            ':' => Some(TokenKind::Colon),
            '.' => Some(TokenKind::Period),
            '?' => Some(TokenKind::QuestionMark),
            '!' => Some(TokenKind::Negate),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Double(f32),
    Bool(bool),
    String(String),
}

impl Literal {
    pub fn as_bool(s: &str) -> Option<Literal> {
        let result = match s {
            "true" => true.into(),
            "false" => false.into(),
            _ => None,
        }?;
        Some(Literal::Bool(result))
    }
}

impl From<Literal> for TokenKind {
    fn from(literal: Literal) -> TokenKind {
        TokenKind::Literal(literal)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Let,
    Fn,
    If,
    Else,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        match s {
            "let" => Keyword::Let.into(),
            "fn" => Keyword::Fn.into(),
            "if" => Keyword::If.into(),
            "else" => Keyword::Else.into(),
            _ => None,
        }
    }
}

impl From<Keyword> for TokenKind {
    fn from(keyword: Keyword) -> TokenKind {
        TokenKind::Keyword(keyword)
    }
}
