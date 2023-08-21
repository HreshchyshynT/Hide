use clap::Parser;

use crate::hide_args::HideArgs;
use crate::words_storage::{InMemoryWordsStorage, WordsStorage};

mod hide_args;
mod words_storage;

fn main() {
    let args = HideArgs::parse();
    let mut storage = InMemoryWordsStorage::new();

    if !args.add_words.is_empty() {
        add_words(&mut storage, &args.add_words);
    }
}

fn add_words(storage: &mut dyn WordsStorage, words: &Vec<String>) {
    println!("adding words...");
    words
        .iter()
        .map(|word| (word, storage.put(word)))
        .map(|(word, result)| match result {
            Ok(()) => format!("saved {}", word),
            Err(error) => format!("{}", error),
        })
        .for_each(|msg| println!("{}", msg));
}
