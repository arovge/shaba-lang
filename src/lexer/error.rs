use super::token::SourcePosition;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnterminatedString {
        position: Range<SourcePosition>,
    },
    UnknownLexme {
        position: Range<SourcePosition>,
        lexme: char,
    },
}

pub enum TokenizeError {
    UnterminatedString,
    UnknownLexme { lexme: char },
}

impl TokenizeError {
    pub fn lexer_err(self, start: SourcePosition, end: SourcePosition) -> LexerError {
        let position = start..end;
        match self {
            TokenizeError::UnterminatedString => LexerError::UnterminatedString { position },
            TokenizeError::UnknownLexme { lexme } => LexerError::UnknownLexme { position, lexme },
        }
    }
}
