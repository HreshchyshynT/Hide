#[allow(dead_code)]
use std::collections::HashSet;

use error::Error;

type Result = std::result::Result<(), Error>;

mod error;

pub trait KeysStorage {
    fn put(&mut self, key: &str) -> Result;
    fn remove(&mut self, key: &str) -> Result;
    fn contains(&self, key: &str) -> bool;
    fn all(&self) -> HashSet<String>;
}

pub struct InMemoryKeysStorage {
    storage: HashSet<String>,
}

impl InMemoryKeysStorage {
    fn new() -> Self {
        InMemoryKeysStorage {
            storage: HashSet::new(),
        }
    }

    pub fn init_with(set: &HashSet<String>) -> Self {
        InMemoryKeysStorage {
            storage: set.to_owned(),
        }
    }
}

impl KeysStorage for InMemoryKeysStorage {
    fn put(&mut self, key: &str) -> Result {
        if key.is_empty() {
            return Err(Error::Other(String::from("Can't save empty string")));
        }
        match self.storage.insert(key.to_string()) {
            true => Ok(()),
            false => Err(Error::KeyAlreadyExists(key.to_string())),
        }
    }

    fn remove(&mut self, key: &str) -> Result {
        match self.storage.remove(key) {
            true => Ok(()),
            false => Err(Error::KeyNotFound(key.to_string())),
        }
    }

    fn contains(&self, key: &str) -> bool {
        self.storage.contains(key)
    }

    fn all(&self) -> HashSet<String> {
        self.storage.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_put_key() {
        let mut storage = InMemoryKeysStorage::new();

        assert!(storage.put("word").is_ok());
        assert!(storage.storage.contains("word"));
    }

    #[test]
    fn test_remove_key() {
        let mut storage = InMemoryKeysStorage::new();

        storage.put("word").unwrap();
        assert!(storage.storage.contains("word"));

        assert!(storage.remove("word").is_ok());
        assert!(!storage.storage.contains("word"));
    }

    #[test]
    fn test_contains_key() {
        let mut storage = InMemoryKeysStorage::new();

        assert!(!storage.contains("word"));

        storage.put("word1").unwrap();
        assert!(storage.contains("word1"));
        assert!(!storage.contains("word2"));

        storage.remove("word1").unwrap();
        assert!(!storage.contains("word1"));
    }

    #[test]
    fn test_put_existing_key() {
        let mut storage = InMemoryKeysStorage::new();

        assert!(storage.put("word").is_ok());
        assert!(storage.put("word").is_err()); // Assuming that putting an existing word returns an error
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut storage = InMemoryKeysStorage::new();

        assert!(storage.remove("word").is_err()); // Assuming that removing a nonexistent word returns an error
    }

    #[test]
    fn test_contains_after_remove() {
        let mut storage = InMemoryKeysStorage::new();

        storage.put("word").unwrap();
        assert!(storage.contains("word"));

        storage.remove("word").unwrap();
        assert!(!storage.contains("word"));
    }

    #[test]
    fn test_put_and_remove_multiple_key() {
        let mut storage = InMemoryKeysStorage::new();

        storage.put("word1").unwrap();
        storage.put("word2").unwrap();
        assert!(storage.contains("word1"));
        assert!(storage.contains("word2"));

        storage.remove("word1").unwrap();
        assert!(!storage.contains("word1"));
        assert!(storage.contains("word2"));
    }

    #[test]
    fn test_put_empty_string() {
        let mut storage = InMemoryKeysStorage::new();

        assert!(storage.put("").is_err()); // Assuming that putting an empty string returns an error
    }

    #[test]
    fn test_remove_empty_string() {
        let mut storage = InMemoryKeysStorage::new();

        assert!(storage.remove("").is_err()); // Assuming that removing an empty string returns an error
    }

    #[test]
    fn test_all_works() {
        let mut storage = InMemoryKeysStorage::new();

        storage.put("one").unwrap();
        storage.put("two").unwrap();

        let all = storage.all();
        assert!(all.contains("one"));
        assert!(all.contains("two"));
        assert_eq!(all.len(), 2);
    }
}
