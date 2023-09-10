mod driver;
mod error;
mod lexer;
pub mod parser;

#[cfg(test)]
mod tests;

use crate::{lexer::lib::Lexer, parser::Parser};
use error::ShabaCompilerError;

fn main() -> Result<(), ShabaCompilerError> {
    let source = driver::read_source()?;
    let mut lexer = Lexer::new(source.as_str());
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    println!("{:?}", ast);

    Ok(())
}
