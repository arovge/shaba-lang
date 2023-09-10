use crate::driver::error::DriverError;
use crate::lexer::error::LexerError;
use crate::parser::error::ParserError;

#[derive(Debug)]
pub enum ShabaCompilerError {
    Driver(DriverError),
    Lexer(LexerError),
    Parser(ParserError),
}

impl From<DriverError> for ShabaCompilerError {
    fn from(e: DriverError) -> ShabaCompilerError {
        ShabaCompilerError::Driver(e)
    }
}

impl From<LexerError> for ShabaCompilerError {
    fn from(e: LexerError) -> ShabaCompilerError {
        ShabaCompilerError::Lexer(e)
    }
}

impl From<ParserError> for ShabaCompilerError {
    fn from(e: ParserError) -> ShabaCompilerError {
        ShabaCompilerError::Parser(e)
    }
}
