use error::ShabaCompilerError;

mod driver;
mod error;
mod lexer;
pub mod parser;

fn main() -> Result<(), ShabaCompilerError> {
    let source = driver::read_source()?;
    let tokens = lexer::tokenize(source.as_str())?;
    let ast = parser::parse(tokens)?;

    println!("{:?}", ast);

    Ok(())
}
