#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    WordAlreadyExists(String),
    WordNotFound(String),
    StorageFull,
    StorageReadError,
    StorageWriteError,
    StorageInitializationError,
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::WordAlreadyExists(word) => {
                write!(f, "Word '{}' already exists in storage", word)
            }
            Error::WordNotFound(word) => write!(f, "Word '{}' not found in storage", word),
            Error::StorageFull => write!(f, "Storage is full"),
            Error::StorageReadError => write!(f, "Error reading from storage"),
            Error::StorageWriteError => write!(f, "Error writing to storage"),
            Error::StorageInitializationError => write!(f, "Error initializing storage"),
            Error::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
