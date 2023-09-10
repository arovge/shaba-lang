#[derive(Debug)]
pub enum ParserError {
    ExpectedToken {
        // source_position: SourcePosition,
        expected_token: ExpectedToken,
    },
    Unknown(String),
}

#[derive(Debug)]
pub enum ExpectedToken {
    ClosingParen,
}
