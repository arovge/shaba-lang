#[derive(Debug, Eq, PartialEq)]
pub struct ParserError {
    pub errors: Vec<ParsingError>,
}

// TODO: Add source location onto this
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ParsingError {
    ExpectedToken(ExpectedToken),
    Unknown(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExpectedToken {
    ClosingParen,
}
