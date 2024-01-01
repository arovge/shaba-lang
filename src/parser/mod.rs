use self::{ast::Node, error::ParserError, parser::Parser};
use crate::lexer::token::Token;

pub mod ast;
pub mod error;
mod parser;
mod scanner;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Node>, ParserError> {
    Parser::new(tokens).parse()
}
