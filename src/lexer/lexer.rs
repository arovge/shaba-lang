use super::{token::{Keyword, Literal, TokenKind}, source::Source};
use crate::lexer::token::Token;

pub struct Lexer {
    source: Source
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Self {
            source: Source::new(source)
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        return tokens;
    }

    fn next_token(&mut self) -> Option<Token> {
        self.source.advance_to_next_token();

        let start: super::token::SourcePosition = self.source.position();
        let token_kind = self.next_token_kind()?;
        let end = self.source.position();

        return Token::new(token_kind, start, end).into();
    }

    fn next_token_kind(&mut self) -> Option<TokenKind> {
        if let Some(token) = self.read_single_char_token() {
            return token.into();
        }

        if let Some(literal) = self.read_literal() {
            return TokenKind::Literal(literal).into();
        }

        if let Some(lexme) = self.read_lexme() {
            return match lexme.as_str() {
                "true" => TokenKind::Literal(Literal::Bool(true)).into(),
                "false" => TokenKind::Literal(Literal::Bool(false)).into(),
                "let" => TokenKind::Keyword(Keyword::Let).into(),
                "var" => TokenKind::Keyword(Keyword::Var).into(),
                _ => TokenKind::Identifier(lexme.to_string()).into(),
            };
        }

        let ch = self.source.next()?;
        let str = String::from(ch);
        return TokenKind::Unknown(str).into();
    }

    fn read_lexme(&mut self) -> Option<String> {
        let ch = self.source.next_if(|ch| is_start_of_identifier(ch))?;

        let mut chars: Vec<char> = vec![ch];
        while let Some(ch) = self.source.next_if(|ch| is_identifier(ch)) {
            chars.push(ch);
        }
        let str = String::from_iter(chars);
        return Some(str);
    }

    fn read_literal(&mut self) -> Option<Literal> {
        if let Some(str) = self.read_str() {
            return Some(Literal::String(str));
        }
        if let Some(num) = self.read_number() {
            return Some(Literal::Integer(num));
        }
        return None;
    }

    fn read_single_char_token(&mut self) -> Option<TokenKind> {
        let ch = self.source.peek()?;
        let token = match ch {
            '=' => Some(TokenKind::Equals),
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
            '!' => Some(TokenKind::ExclaimationPoint),
            _ => None,
        }?;
        self.source.next();
        return token.into();
    }

    fn read_str(&mut self) -> Option<String> {
        self.source.next_if(|ch| ch == '"')?;
        let mut chars: Vec<char> = vec![];

        while let Some(ch) = self.source.next_if(|ch| ch != '"') {
            chars.push(ch);
        }

        let is_valid: bool = {
            match self.source.peek() {
                Some('"') => true,
                _ => false,
            }
        };

        // TODO: Error handling for malformed strings
        if !is_valid {}

        self.source.next();
        return String::from_iter(chars).into();
    }

    fn read_number(&mut self) -> Option<i32> {
        let first_ch = self.source.next_if(|ch| ch.is_ascii_digit())?;
        let mut chars: Vec<char> = vec![first_ch];

        while let Some(ch) = self.source.next_if(|ch| ch.is_ascii_digit()) {
            chars.push(ch);
        }

        let str = String::from_iter(chars);
        return str.parse::<i32>().ok();
    }
}

fn is_start_of_identifier(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier(c: char) -> bool {
    is_start_of_identifier(c) || c.is_ascii_digit()
}
