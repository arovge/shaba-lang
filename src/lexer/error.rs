use super::token::SourceLocation;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct LexerError {
    error: TokenizeError,
    location: Range<SourceLocation>,
}

impl LexerError {
    pub fn new(error: TokenizeError, start: SourceLocation, end: SourceLocation) -> Self {
        Self {
            error,
            location: start..end,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    UnterminatedString,
    UnknownLexme(char),
}
