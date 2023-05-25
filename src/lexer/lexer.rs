use std::{str::CharIndices, iter::Peekable};

use super::token::{Keyword, Literal, TokenKind, SourcePosition};
use crate::lexer::token::Token;

pub struct Lexer<'a> {
    chars: Peekable<CharIndices<'a>>,
    line_position: u32,
    column_position: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        let chars: Peekable<CharIndices<'a>> = source
            .char_indices()
            .peekable();
        Self {
            chars,
            line_position: 1,
            column_position: 1
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
        self.eat_whitespace();

        let start = self.source_position();
        let token_kind = self.next_token_kind()?;
        let end = self.source_position();

        return Token::new(token_kind, start, end).into();
    }

    fn next_token_kind(&mut self) -> Option<TokenKind> {
        if let Some(token) = self.read_single_char_token() {
            return token.into();
        }

        if let Some(literal) = self.read_literal() {
            return TokenKind::Literal(literal).into();
        }

        if let Some(chunk) = self.read_chunk() {
            return match chunk.as_str() {
                "true" => TokenKind::Literal(Literal::Bool(true)).into(),
                "false" => TokenKind::Literal(Literal::Bool(false)).into(),
                "let" => TokenKind::Keyword(Keyword::Let).into(),
                "var" => TokenKind::Keyword(Keyword::Var).into(),
                _ => TokenKind::Identifier(chunk.to_string()).into(),
            };
        }

        let (_, ch) = self.next()?;
        let str = String::from(ch);
        return TokenKind::Unknown(str).into();
    }

    // There is likely a better name for what this returns
    // Returns possible identifiers, boolean literals, etc
    fn read_chunk(&mut self) -> Option<String> {
        let ch = self.next_if(|ch| is_start_of_identifier(ch))?;

        let mut chars: Vec<char> = vec![ch.1];
        while let Some(ch) = self.next_if(|ch| is_identifier(ch)) {
            chars.push(ch.1);
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
        let (_, ch) = self.peek()?;
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
        self.next();
        return token.into();
    }

    fn read_str(&mut self) -> Option<String> {
        self.next_if(|ch| ch == '"')?;
        let mut chars: Vec<char> = vec![];

        while let Some(ch) = self.next_if(|ch| ch != '"') {
            chars.push(ch.1);
        }

        let is_valid: bool = {
            match self.peek() {
                Some((_, '"')) => true,
                _ => false
            }
        };

        // TODO: Error handling for malformed strings
        if !is_valid {

        }

        self.next();
        return String::from_iter(chars).into();
    }

    fn read_number(&mut self) -> Option<i32> {
        let first_ch = self.next_if(|ch| ch.is_ascii_digit())?;
        let mut chars: Vec<char> = vec![first_ch.1];

        while let Some(ch) = self.next_if(|ch| ch.is_ascii_digit()) {
            chars.push(ch.1);
        }

        let str = String::from_iter(chars);
        return str.parse::<i32>().ok();
    }

    fn eat_whitespace(&mut self) {
        while let Some(_) = self.next_if(|ch| ch.is_ascii_whitespace()) {}
    }

    fn eat_while(&mut self, condition: fn(char) -> bool) {
        while let Some(_) = self.next_if(condition) {}
    }

    fn source_position(&self) -> SourcePosition {
        SourcePosition::new(self.line_position, self.column_position)
    }

    fn peek(&mut self) -> Option<&(usize, char)> {
        self.chars.peek()
    }

    fn next_if(&mut self, condition: fn(char) -> bool) -> Option<(usize, char)> {
        let next = self.chars.next_if(|ch| condition(ch.1))?;

        if next.1 == '\n' {
            self.line_position += 1;
            self.column_position = 1;
        } else {
            self.column_position += 1;
        }

        return Some(next);
    }

    fn next(&mut self) -> Option<(usize, char)> {
        let next = self.chars.next()?;

        if next.1 == '\n' {
            self.line_position += 1;
            self.column_position = 1;
        } else {
            self.column_position += 1;
        }

        return Some(next);
    }
}

fn is_start_of_identifier(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier(c: char) -> bool {
    is_start_of_identifier(c) || c.is_ascii_digit()
}
