use std::fmt;

#[derive(Debug)]
pub enum DriverError {
    MissingFileFlag,
    UnableToRead,
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverError::MissingFileFlag => write!(f, "Missing `--file` flag"),
            DriverError::UnableToRead => write!(f, "Unable to read file"),
        }
    }
}
