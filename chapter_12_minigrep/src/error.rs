use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RunError {
    InputOutputError(std::io::Error),
    Other(Box<dyn Error>),
}

impl Display for RunError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RunError::InputOutputError(error) => {
                write!(formatter, "{error}")
            }
            RunError::Other(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for RunError {}

impl From<std::io::Error> for RunError {
    fn from(error: std::io::Error) -> Self {
        RunError::InputOutputError(error)
    }
}
