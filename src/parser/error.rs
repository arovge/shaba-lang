use crate::lexer::token::SourceLocation;
use std::ops::Range;

#[derive(Debug)]
pub struct ParserError {
    errors: Vec<ParsingError>,
    location: Range<SourceLocation>,
}

impl ParserError {
    pub fn new(errors: Vec<ParsingError>, start: SourceLocation, end: SourceLocation) -> Self {
        Self {
            errors,
            location: start..end,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    ExpectedToken(ExpectedToken),
    Unknown(String),
}

#[derive(Debug, Clone)]
pub enum ExpectedToken {
    ClosingParen,
}
