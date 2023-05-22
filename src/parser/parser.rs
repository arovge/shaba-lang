use crate::lexer::token::Token;

use super::error::ParserError;

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens
        }
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        todo!();
    }
}
