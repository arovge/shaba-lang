use crate::driver::error::DriverError;
use crate::lexer::error::LexerError;
use crate::parser::error::ParserError;

#[derive(Debug)]
pub enum ShabaCompilerError {
    Driver,
    Lexer,
    Parser,
}

impl From<DriverError> for ShabaCompilerError {
    fn from(_e: DriverError) -> ShabaCompilerError {
        ShabaCompilerError::Driver
    }
}

impl From<LexerError> for ShabaCompilerError {
    fn from(_e: LexerError) -> ShabaCompilerError {
        ShabaCompilerError::Lexer
    }
}

impl From<ParserError> for ShabaCompilerError {
    fn from(_e: ParserError) -> ShabaCompilerError {
        ShabaCompilerError::Parser
    }
}
