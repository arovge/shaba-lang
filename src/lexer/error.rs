use super::token::SourceLocation;
use std::ops::Range;

#[derive(Debug, PartialEq)]
pub struct LexerError {
    error: TokenizeError,
    location: Range<SourceLocation>,
}

#[derive(Debug, PartialEq)]
pub enum TokenizeError {
    UnterminatedString,
    UnknownLexme(char),
}

impl TokenizeError {
    pub fn into_lexer_err(self, start: SourceLocation, end: SourceLocation) -> LexerError {
        let location = start..end;
        LexerError {
            error: self,
            location,
        }
    }
}
