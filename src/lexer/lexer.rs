use super::{token::{Keyword, Literal, TokenKind}, source::Source, error::LexerError};
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::<Token>::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        return Ok(tokens);
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.source.advance_to_next_token();

        let start = self.source.position();
        let Some(token_kind) = self.next_token_kind()? else {
            return Ok(None);
        };
        let end = self.source.position();

        Ok(Some(Token::new(token_kind, start, end)))
    }

    fn next_token_kind(&mut self) -> Result<Option<TokenKind>, LexerError> {
        if let Some(token) = self.read_single_char_token() {
            return Ok(Some(token));
        }

        if let Some(literal) = self.read_literal()? {
            return Ok(Some(TokenKind::Literal(literal)));
        }

        if let Some(lexme) = self.read_lexme() {
            return match lexme.as_str() {
                "true" => Ok(Some(TokenKind::Literal(Literal::Bool(true)))),
                "false" => Ok(Some(TokenKind::Literal(Literal::Bool(false)))),
                "let" => Ok(Some(TokenKind::Keyword(Keyword::Let))),
                "var" => Ok(Some(TokenKind::Keyword(Keyword::Var))),
                _ => Ok(Some(TokenKind::Identifier(lexme.to_string()))),
            };
        }

        let Some(ch) = self.source.next() else { return Ok(None) };
        let str = String::from(ch);
        return Ok(Some(TokenKind::Unknown(str)));
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

    fn read_literal(&mut self) -> Result<Option<Literal>, LexerError> {
        if let Some(str) = self.read_str()? {
            return Ok(Some(Literal::String(str)));
        }
        if let Some(num) = self.read_number() {
            return Ok(Some(Literal::Integer(num)));
        }
        return Ok(None);
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

    fn read_str(&mut self) -> Result<Option<String>, LexerError> {
        let Some(next) = self.source.next_if(|ch| ch == '"') else {
            return Ok(None);
        };
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

        // TODO: Include source positon
        if !is_valid {
            return Err(LexerError::UnterminatedString);
        }

        self.source.next();
        return Ok(Some(String::from_iter(chars)));
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
