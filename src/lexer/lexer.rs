use super::{
    error::{LexerError, LexingError},
    scanner::Scanner,
    token::{Keyword, Literal, TokenKind},
};
use crate::lexer::token::Token;

pub struct Lexer {
    scanner: Scanner,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        let scanner = Scanner::new(source);

        Self { scanner }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        assert!(self.scanner.is_eof(), "Not at end of file");
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.scanner.advance_to_next_token();

        let start = self.scanner.location();
        let next_token_kind = self.next_token_kind();
        let end = self.scanner.location();

        match next_token_kind {
            Ok(Some(token_kind)) => Ok(Some(Token::new(token_kind, start, end))),
            Ok(None) => Ok(None),
            Err(e) => Err(LexerError::new(e, start, end)),
        }
    }

    fn next_token_kind(&mut self) -> Result<Option<TokenKind>, LexingError> {
        let single_char_token = self.read_single_char_token();
        if single_char_token.is_some() {
            return Ok(single_char_token);
        }

        let literal = self.read_literal()?;
        if literal.is_some() {
            return Ok(literal);
        }

        let lexme = self.read_lexme();
        if lexme.is_some() {
            return Ok(lexme);
        }

        let Some(lexme) = self.scanner.next() else {
            return Ok(None);
        };
        Err(LexingError::UnknownLexme(lexme))
    }

    fn read_lexme(&mut self) -> Option<TokenKind> {
        let ch = self.scanner.next_if(is_start_of_identifier)?;

        let mut chars: Vec<char> = vec![ch];
        while let Some(ch) = self.scanner.next_if(is_identifier) {
            chars.push(ch);
        }
        let lexme = &String::from_iter(chars);

        if let Some(literal) = Literal::as_bool(lexme) {
            return TokenKind::Literal(literal).into();
        }

        if let Some(keyword) = Keyword::from_str(lexme) {
            return TokenKind::Keyword(keyword).into();
        }

        TokenKind::Identifier(lexme.to_string()).into()
    }

    // TODO: this fn name makes no sense anymore
    // Find a better way to do this
    // The token should be responsible for trying to map a string into Option<TokenKind>
    // This should only read stuff and present it to that fn
    fn read_single_char_token(&mut self) -> Option<TokenKind> {
        let token_kind = self.scanner.next_if_map(TokenKind::from_char)?;

        let is_maybe_double_char_token = matches!(token_kind, TokenKind::Eq)
            || matches!(token_kind, TokenKind::GreaterThan)
            || matches!(token_kind, TokenKind::LessThan)
            || matches!(token_kind, TokenKind::Negate);

        if !is_maybe_double_char_token {
            return Some(token_kind);
        }

        if self.scanner.next_if(|x| x == '=').is_some() {
            match token_kind {
                TokenKind::Eq => Some(TokenKind::EqEq),
                TokenKind::GreaterThan => Some(TokenKind::GreaterThanEq),
                TokenKind::LessThan => Some(TokenKind::LessThanEq),
                TokenKind::Negate => Some(TokenKind::NotEq),
                _ => Some(token_kind),
            }
        } else {
            Some(token_kind)
        }
    }

    fn read_literal(&mut self) -> Result<Option<TokenKind>, LexingError> {
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

    fn read_str(&mut self) -> Result<Option<String>, LexingError> {
        if self.scanner.next_if(|ch| ch == '"').is_none() {
            return Ok(None);
        }

        let str = self
            .scanner
            .take_while(|ch| ch != '"')
            .unwrap_or(String::from(""));

        let is_unterminated = self.scanner.peek() != Some('"');

        if is_unterminated {
            return Err(LexingError::UnterminatedString);
        }

        self.scanner.next();
        Ok(Some(str))
    }

    fn read_number(&mut self) -> Option<Literal> {
        let num = self
            .scanner
            .take_while(|ch| ch.is_ascii_digit())?
            .parse::<i32>()
            .ok()?;
        Some(Literal::Int(num))
    }
}

fn is_start_of_identifier(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_identifier(c: char) -> bool {
    is_start_of_identifier(c) || c.is_ascii_digit()
}
