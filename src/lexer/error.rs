use super::token::SourceLocation;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct LexerError {
    location: Range<SourceLocation>,
    error: LexingError
}

impl LexerError {
    pub fn new(location: Range<SourceLocation>, error: LexingError) -> Self {
        Self {
            location,
            error
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexingError {
    UnterminatedString,
    UnknownLexme(char)
}

impl LexingError {
    pub fn lexer_err(self, start: SourceLocation, end: SourceLocation) -> LexerError {
        let location = start..end;
        LexerError::new(location, self)
    }
}
