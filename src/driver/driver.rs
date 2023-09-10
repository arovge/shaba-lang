use crate::driver::error::DriverError;
use std::{env, fs};

pub fn read_source() -> Result<String, DriverError> {
    let Some(file_path) = get_file_flag() else {
        return Err(DriverError::MissingFileFlag);
    };
    let content = fs::read_to_string(file_path);
    return content.or(Err(DriverError::UnableToRead));
}

fn get_file_flag() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let index = args.iter().position(|arg| arg.eq(&"--file"))?;
    let file_name = args.get(index + 1)?;
    Some(file_name.clone())
}
