#[allow(dead_code)]
use std::collections::HashSet;

use error::Error;

type Result = std::result::Result<(), Error>;

mod error;

pub trait WordsStorage {
    fn put(&mut self, word: &str) -> Result;
    fn remove(&mut self, word: &str) -> Result;
    fn contains(&self, word: &str) -> bool;
}

pub struct InMemoryWordsStorage {
    storage: HashSet<String>,
}

impl InMemoryWordsStorage {
    fn new() -> InMemoryWordsStorage {
        let storage: HashSet<String> = HashSet::new();
        let storage = InMemoryWordsStorage { storage };
        storage
    }
}

impl WordsStorage for InMemoryWordsStorage {
    fn put(&mut self, word: &str) -> Result {
        if word.is_empty() {
            return Err(Error::Other(String::from("Can't save empty string")));
        }
        match self.storage.insert(word.to_string()) {
            true => Ok(()),
            false => Err(Error::WordAlreadyExists(word.to_string())),
        }
    }

    fn remove(&mut self, word: &str) -> Result {
        match self.storage.remove(word) {
            true => Ok(()),
            false => Err(Error::WordNotFound(word.to_string())),
        }
    }

    fn contains(&self, word: &str) -> bool {
        self.storage.contains(word)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_put_word() {
        let mut storage = InMemoryWordsStorage::new();

        assert!(storage.put("word").is_ok());
        assert!(storage.storage.contains("word"));
    }

    #[test]
    fn test_remove_word() {
        let mut storage = InMemoryWordsStorage::new();

        storage.put("word").unwrap();
        assert!(storage.storage.contains("word"));

        assert!(storage.remove("word").is_ok());
        assert!(!storage.storage.contains("word"));
    }

    #[test]
    fn test_contains_word() {
        let mut storage = InMemoryWordsStorage::new();

        assert!(!storage.contains("word"));

        storage.put("word1").unwrap();
        assert!(storage.contains("word1"));
        assert!(!storage.contains("word2"));

        storage.remove("word1").unwrap();
        assert!(!storage.contains("word1"));
    }

    #[test]
    fn test_put_existing_word() {
        let mut storage = InMemoryWordsStorage::new();

        assert!(storage.put("word").is_ok());
        assert!(storage.put("word").is_err()); // Assuming that putting an existing word returns an error
    }

    #[test]
    fn test_remove_nonexistent_word() {
        let mut storage = InMemoryWordsStorage::new();

        assert!(storage.remove("word").is_err()); // Assuming that removing a nonexistent word returns an error
    }

    #[test]
    fn test_contains_after_remove() {
        let mut storage = InMemoryWordsStorage::new();

        storage.put("word").unwrap();
        assert!(storage.contains("word"));

        storage.remove("word").unwrap();
        assert!(!storage.contains("word"));
    }

    #[test]
    fn test_put_and_remove_multiple_words() {
        let mut storage = InMemoryWordsStorage::new();

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
        let mut storage = InMemoryWordsStorage::new();

        assert!(storage.put("").is_err()); // Assuming that putting an empty string returns an error
    }

    #[test]
    fn test_remove_empty_string() {
        let mut storage = InMemoryWordsStorage::new();

        assert!(storage.remove("").is_err()); // Assuming that removing an empty string returns an error
    }
}
