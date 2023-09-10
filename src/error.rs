use crate::driver::error::DriverError;
use crate::lexer::error::LexerError;
use crate::parser::error::ParserError;

#[derive(Debug)]
pub enum ShabaCompilerError {
    DriverError(DriverError),
    LexerError(LexerError),
    ParserError(ParserError),
}

impl From<DriverError> for ShabaCompilerError {
    fn from(e: DriverError) -> ShabaCompilerError {
        ShabaCompilerError::DriverError(e)
    }
}

impl From<LexerError> for ShabaCompilerError {
    fn from(e: LexerError) -> ShabaCompilerError {
        ShabaCompilerError::LexerError(e)
    }
}

impl From<ParserError> for ShabaCompilerError {
    fn from(e: ParserError) -> ShabaCompilerError {
        ShabaCompilerError::ParserError(e)
    }
}
