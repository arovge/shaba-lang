use super::{
    error::{LexerError, TokenizeError},
    source::Source,
    token::{Keyword, Literal, TokenKind},
};
use crate::lexer::token::Token;

pub struct Lexer {
    source: Source,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        Self {
            source: Source::new(source),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        return Ok(tokens);
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.source.advance_to_next_token();

        let start = self.source.position();
        let next_token_kind = self.next_token_kind();
        let end = self.source.position();

        match next_token_kind {
            Ok(Some(token_kind)) => Ok(Some(Token::new(token_kind, start, end))),
            Ok(None) => Ok(None),
            Err(e) => Err(e.lexer_err(start, end)),
        }
    }

    fn next_token_kind(&mut self) -> Result<Option<TokenKind>, TokenizeError> {
        let token_kind = self
            .read_single_char_token()
            .or(self.read_literal()?)
            .or(self.read_lexme());

        if token_kind.is_some() {
            return Ok(token_kind);
        }

        let Some(lexme) = self.source.next() else { return Ok(None) };
        return Err(TokenizeError::UnknownLexme { lexme });
    }

    fn read_lexme(&mut self) -> Option<TokenKind> {
        let ch = self.source.next_if(|ch| is_start_of_identifier(ch))?;

        let mut chars: Vec<char> = vec![ch];
        while let Some(ch) = self.source.next_if(|ch| is_identifier(ch)) {
            chars.push(ch);
        }
        let lexme = String::from_iter(chars);
        let token_kind = match lexme.as_str() {
            "true" => TokenKind::Literal(Literal::Bool(true)),
            "false" => TokenKind::Literal(Literal::Bool(false)),
            "let" => TokenKind::Keyword(Keyword::Let),
            "var" => TokenKind::Keyword(Keyword::Var),
            _ => TokenKind::Identifier(lexme.to_string()),
        };
        return Some(token_kind);
    }

    fn read_single_char_token(&mut self) -> Option<TokenKind> {
        let ch = self.source.peek()?;
        let token = match ch {
            '=' => Some(TokenKind::Equals),
            '+' => Some(TokenKind::Plus),
            '-' => Some(TokenKind::Minus),
            '*' => Some(TokenKind::Asterisk),
            '/' => Some(TokenKind::Slash),
            '>' => {
                if self.source.peek_next() == Some('=') {
                    Some(TokenKind::GreaterThanEqual)
                } else {
                    Some(TokenKind::GreaterThan)
                }
            }
            '<' => {
                if self.source.peek_next() == Some('=') {
                    Some(TokenKind::LessThanEqual)
                } else {
                    Some(TokenKind::LessThan)
                }
            }
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

    fn read_literal(&mut self) -> Result<Option<TokenKind>, TokenizeError> {
        if let Some(str) = self.read_str()? {
            let token_kind = TokenKind::Literal(Literal::String(str));
            return Ok(Some(token_kind));
        }
        if let Some(num) = self.read_number() {
            let token_kind = TokenKind::Literal(num);
            return Ok(Some(token_kind));
        }
        Ok(None)
    }

    fn read_str(&mut self) -> Result<Option<String>, TokenizeError> {
        let Some(_) = self.source.next_if(|ch| ch == '"') else {
            return Ok(None);
        };

        let str = self
            .source
            .take_while(|ch| ch != '"')
            .unwrap_or(String::from(""));

        let is_unterminated = self.source.peek() != Some('"');

        if is_unterminated {
            return Err(TokenizeError::UnterminatedString);
        }

        self.source.next();
        return Ok(Some(str));
    }

    fn read_number(&mut self) -> Option<Literal> {
        let num = self
            .source
            .take_while(|ch| ch.is_ascii_digit())?
            .parse::<i32>()
            .ok()?;
        return Some(Literal::Integer(num));
    }
}

fn is_start_of_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier(c: char) -> bool {
    is_start_of_identifier(c) || c.is_ascii_digit()
}
