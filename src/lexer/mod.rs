use crate::lexer::lexer::Lexer;

use self::{error::LexerError, token::Token};

pub mod error;
pub mod lexer;
pub mod scanner;
pub mod token;

pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
    Lexer::new(source).tokenize()
}
