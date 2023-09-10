#[derive(Debug)]
pub struct ParserError {
    errors: Vec<ParsingError>
    // source_position: SourcePosition
}

impl ParserError {
    pub fn new(errors: Vec<ParsingError>) -> Self {
        Self {
            errors
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParsingError {
    ExpectedToken(ExpectedToken),
    Unknown(String)
}

#[derive(Debug, Clone)]
pub enum ExpectedToken {
    ClosingParen,
}
