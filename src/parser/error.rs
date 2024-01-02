#[derive(Debug)]
pub struct ParserError {
    errors: Vec<ParsingError>,
}

impl ParserError {
    pub fn new(errors: Vec<ParsingError>) -> Self {
        Self { errors }
    }
}

// TODO: Add source location onto this
#[derive(Debug, Clone)]
pub enum ParsingError {
    ExpectedToken(ExpectedToken),
    Unknown(String),
}

#[derive(Debug, Clone)]
pub enum ExpectedToken {
    ClosingParen,
}
