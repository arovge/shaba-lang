use crate::driver::error::DriverError;
use crate::lexer::error::LexerError;
use crate::parser::error::ParserError;

#[derive(Debug)]
pub enum ShabaCompilerError {
    DriverError,
    LexerError,
    ParserError
}

impl From<DriverError> for ShabaCompilerError {
    fn from(_e: DriverError) -> ShabaCompilerError {
        ShabaCompilerError::DriverError
    }
}

impl From<LexerError> for ShabaCompilerError {
    fn from(_e: LexerError) -> ShabaCompilerError {
        ShabaCompilerError::LexerError
    }
}

impl From<ParserError> for ShabaCompilerError {
    fn from(_e: ParserError) -> ShabaCompilerError {
        ShabaCompilerError::ParserError
    }
}
