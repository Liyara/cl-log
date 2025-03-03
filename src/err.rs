use std::fmt;

pub enum LoggerError {
    BuildError(String),
}

impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoggerError::BuildError(message) => write!(f, "Logger Error: {}", message),
        }
    }
}