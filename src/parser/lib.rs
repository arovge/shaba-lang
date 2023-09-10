use crate::lexer::token::Token;

use super::error::ParserError;

pub struct Parser {
    _tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { _tokens: tokens }
    }

    pub fn parse(&mut self) -> Result<(), ParserError> {
        todo!();
    }
}
