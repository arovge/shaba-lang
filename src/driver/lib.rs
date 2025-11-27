use crate::driver::error::DriverError;
use std::{env, fs::read_to_string};

pub fn read_source() -> Result<String, DriverError> {
    let file_path = get_file_flag().ok_or(DriverError::MissingFileFlag)?;
    read_to_string(file_path).or(Err(DriverError::UnableToRead))
}

fn get_file_flag() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    let index = args.iter().position(|arg| arg.eq(&"--file"))?;
    let file_name = args.get(index + 1)?;
    Some(file_name.clone())
}
