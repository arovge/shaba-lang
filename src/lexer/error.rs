use super::token::SourceLocation;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct LexerError {
    error: LexingError,
    location: Range<SourceLocation>,
}

impl LexerError {
    pub fn new(error: LexingError, start: SourceLocation, end: SourceLocation) -> Self {
        Self {
            error,
            location: start..end,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexingError {
    UnterminatedString,
    UnknownLexme(char),
}
