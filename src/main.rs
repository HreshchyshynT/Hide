use crate::hide_args::HideArgs;
use crate::words_storage::{InMemoryWordsStorage, WordsStorage};
use clap::Parser;
use serde_json::{json, value, Map, Value};
use simple_logger::SimpleLogger;
use std::fs;

mod hide_args;
mod words_storage;

fn main() {
    let args = HideArgs::parse();

    // init logger if debug enabled
    if args.debug {
        SimpleLogger::new().init().unwrap();
        log::info!("debug enabled, logger initialized.");
    }

    let mut storage = InMemoryWordsStorage::new();

    // add words if any
    if !args.add_words.is_empty() {
        add_words(&mut storage, &args.add_words);
    }

    // remove words if any
    if !args.remove_words.is_empty() {
        remove_words(&mut storage, &args.remove_words);
    }

    // nothing to do if input not specified
    if let None = args.input_file {
        return;
    }

    let input_path = args.input_file.unwrap();

    if !input_path.exists() || !input_path.is_file() {
        panic!("File not exists or not a file.")
    }

    let input_path = input_path.to_str().unwrap();

    let file_str = fs::read_to_string(input_path).expect("could not read file");

    let map: Map<String, Value> = serde_json::from_str::<Value>(&file_str)
        .unwrap()
        .as_object()
        .unwrap()
        .clone();

    let output = hide_keys(&storage, &map);

    // TODO: output to file if path specified
    // print otherwise
    println!("{:?}", output);
}

fn add_words<S: WordsStorage>(storage: &mut S, words: &Vec<String>) {
    log::debug!("adding words...");
    words
        .iter()
        .map(|word| (word, storage.put(word)))
        .map(|(word, result)| match result {
            Ok(()) => format!("saved {}", word),
            Err(error) => format!("{}", error),
        })
        .for_each(|msg| log::debug!("{}", msg));
}

fn remove_words<S: WordsStorage>(storage: &mut S, words: &Vec<String>) {
    log::debug!("removing words...");
    words
        .iter()
        .map(|word| (word, storage.remove(word)))
        .map(|(word, result)| match result {
            Ok(()) => format!("removed {}", word),
            Err(error) => format!("{}", error),
        })
        .for_each(|msg| log::debug!("{}", msg));
}

fn hide_keys<S: WordsStorage>(storage: &S, json_map: &Map<String, Value>) -> Map<String, Value> {
    let mut result_map = serde_json::Map::with_capacity(json_map.len());
    for (key, value) in json_map {
        log::debug!("key: {}, value: {}", key, value);
        let value = if value.is_object() {
            // TODO: fix Object({})
            json!(hide_keys(storage, value.as_object().unwrap()))
        } else if storage.contains(key) {
            // TODO: fix String("hidden")
            value::to_value("hidden").unwrap()
        } else {
            value.clone()
        };
        result_map.insert(key.to_owned(), value);
    }
    result_map
}
