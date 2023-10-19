#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    KeyAlreadyExists(String),
    KeyNotFound(String),
    StorageFull,
    StorageReadError,
    StorageWriteError,
    StorageInitializationError,
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::KeyAlreadyExists(key) => {
                write!(f, "Key '{}' already exists in storage", key)
            }
            Error::KeyNotFound(key) => write!(f, "Key '{}' not found in storage", key),
            Error::StorageFull => write!(f, "Storage is full"),
            Error::StorageReadError => write!(f, "Error reading from storage"),
            Error::StorageWriteError => write!(f, "Error writing to storage"),
            Error::StorageInitializationError => write!(f, "Error initializing storage"),
            Error::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}
