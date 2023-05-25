mod driver;
mod lexer;
mod error;
pub mod parser;

#[cfg(test)]
mod tests;

use error::ShabaCompilerError;
use crate::{lexer::Lexer, parser::Parser};

fn main()-> Result<(), ShabaCompilerError> {
    let source = driver::read_source()?;
    let mut lexer = Lexer::new(source.as_str());
    let tokens = lexer.tokenize();

    dbg!(tokens.clone());

    let mut _parser = Parser::new(tokens);

    Ok(())
}
