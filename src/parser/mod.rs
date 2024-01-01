use self::{ast::Node, error::ParserError};
use crate::lexer::token::Token;

pub mod ast;
pub mod error;
mod lib;
use lib::Parser;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParserError> {
    Parser::new(tokens).parse()
}
