use crate::hide_args::HideArgs;
use crate::words_storage::{InMemoryWordsStorage, WordsStorage};
use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{json, Map, Value};
use simple_logger::SimpleLogger;
use std::fs;

mod hide_args;
mod words_storage;

fn main() -> Result<()> {
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
        return Ok(());
    }

    let input_path = args.input_file.unwrap();

    let input_path = input_path.to_str().unwrap();

    let file_str = fs::read_to_string(input_path)
        .with_context(|| format!("could not read file: {}", input_path))?;

    let input_map: Map<String, Value> = serde_json::from_str::<Value>(&file_str)
        .with_context(|| format!("could not parse file: {}", input_path))?
        .as_object()
        .unwrap()
        .clone();

    log::debug!("input map:\n{:?}", input_map);

    let output = hide_keys(&storage, &input_map);
    let output = serde_json::to_string_pretty(&output).unwrap();
    match args.output_file {
        // print to console if output file not specified
        None => println!("{output}"),
        // write to file
        Some(path) => fs::write(path, output).unwrap(),
    };
    Ok(())
}

fn add_words(storage: &mut impl WordsStorage, words: &Vec<String>) {
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

fn remove_words(storage: &mut impl WordsStorage, words: &Vec<String>) {
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

fn hide_keys(storage: &impl WordsStorage, json_map: &Map<String, Value>) -> Map<String, Value> {
    let mut result_map = serde_json::Map::with_capacity(json_map.len());
    for (key, value) in json_map {
        log::debug!("key: {}, value: {}", key, value);
        let value = if value.is_object() {
            json!(hide_keys(storage, value.as_object().unwrap()))
        } else if storage.contains(key) {
            Value::String("hidden".to_string())
        } else {
            value.clone()
        };
        result_map.insert(key.to_owned(), value);
    }
    result_map
}
