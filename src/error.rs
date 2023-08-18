use std::fmt::{self, write};

#[derive(Debug)]
pub enum ArgParseError {
    MissingInputFile,
    MissingWords,
    MissingOutputFile,
    UnknownOption(String),
}

impl fmt::Display for ArgParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgParseError::MissingInputFile => write!(f, "Missing input file"),
            ArgParseError::MissingWords => write!(f, "Missing words after --add-words"),
            ArgParseError::MissingOutputFile => write!(f, "Missing file path after -o"),
            ArgParseError::UnknownOption(msg) => write!(f, "Unknown option: {}", msg),
        }
    }
}

impl std::error::Error for ArgParseError {}
