use crate::config::Config;
use crate::hide_args::HideArgs;
use crate::keys_storage::{InMemoryKeysStorage, KeysStorage};
use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{json, Map, Value};
use simple_logger::SimpleLogger;
use std::collections::HashSet;
use std::fs;

mod config;
mod hide_args;
mod keys_storage;

fn main() -> Result<()> {
    let args = HideArgs::parse();

    // init logger if debug enabled
    if args.debug {
        SimpleLogger::new().init().unwrap();
        log::info!("debug enabled, logger initialized.");
    }

    let config: Config =
        confy::load("hide", "hide-cfg").with_context(|| "could not parse config")?;
    let sensitive_keys = config.sensitive_keys.unwrap_or(HashSet::new());
    let mut storage = InMemoryKeysStorage::init_with(&sensitive_keys);

    // add keys if any
    if !args.add_keys.is_empty() {
        add_keys(&mut storage, &args.add_keys);
    }

    // remove keys if any
    if !args.remove_keys.is_empty() {
        remove_keys(&mut storage, &args.remove_keys);
    }

    if !args.remove_keys.is_empty() || !args.add_keys.is_empty() {
        log::info!("storing config...");
        let config = Config {
            sensitive_keys: Some(storage.all()),
        };
        confy::store("hide", "hide-cfg", &config)
            .with_context(|| "could not store config")
            .unwrap();
    }

    // nothing to do if input not specified
    if let None = args.input_file {
        return Ok(());
    }

    let input_path = args.input_file.unwrap();

    let input_path = input_path.to_str().unwrap();

    let file_str = fs::read_to_string(input_path)
        .with_context(|| format!("could not read file: {}", input_path))?;

    let input = serde_json::from_str::<Value>(&file_str)
        .with_context(|| format!("could not parse file: {}", input_path))?;

    log::debug!("input:\n{:?}", input);

    let output = hide_by_keys(&storage, &input);
    let output = serde_json::to_string_pretty(&output).unwrap();
    match args.output_file {
        // print to console if output file not specified
        None => println!("{output}"),
        // write to file
        Some(path) => fs::write(path, output).unwrap(),
    };
    Ok(())
}

fn add_keys(storage: &mut impl KeysStorage, keys: &Vec<String>) {
    log::debug!("adding keys...");
    keys.iter()
        .map(|key| (key, storage.put(key)))
        .map(|(key, result)| match result {
            Ok(()) => format!("saved {}", key),
            Err(error) => format!("{}", error),
        })
        .for_each(|msg| log::debug!("{}", msg));
}

fn remove_keys(storage: &mut impl KeysStorage, keys: &Vec<String>) {
    log::debug!("removing keys...");
    keys.iter()
        .map(|keys| (keys, storage.remove(keys)))
        .map(|(keys, result)| match result {
            Ok(()) => format!("removed {}", keys),
            Err(error) => format!("{}", error),
        })
        .for_each(|msg| log::debug!("{}", msg));
}

fn hide_by_keys(storage: &impl KeysStorage, json: &Value) -> Value {
    match json {
        Value::Array(_) => hide_by_keys_in_array(storage, json.as_array().unwrap()),
        Value::Object(_) => hide_by_keys_in_map(storage, json.as_object().unwrap()),
        _ => json.clone(),
    }
}

fn hide_by_keys_in_map(storage: &impl KeysStorage, json: &Map<String, Value>) -> Value {
    let mut result_map = serde_json::Map::with_capacity(json.len());
    for (key, value) in json {
        log::debug!("key: {}, value: {}", key, value);
        let value = if storage.contains(key) {
            hide(value)
        } else if value.is_object() {
            hide_by_keys_in_map(storage, value.as_object().unwrap())
        } else if value.is_array() {
            hide_by_keys_in_array(storage, &value.as_array().unwrap())
        } else {
            value.clone()
        };
        result_map.insert(key.to_owned(), value);
    }
    json!(result_map)
}

fn hide_by_keys_in_array(storage: &impl KeysStorage, json: &Vec<Value>) -> Value {
    let mut result: Vec<Value> = Vec::with_capacity(json.len());
    for item in json {
        let item = match item {
            Value::Array(_) => hide_by_keys_in_array(storage, item.as_array().unwrap()),
            Value::Object(_) => hide_by_keys_in_map(storage, item.as_object().unwrap()),
            _ => item.clone(),
        };
        result.push(item);
    }
    json!(result)
}

fn hide(value: &Value) -> Value {
    let new_value = match value {
        Value::Null => Value::Null,
        Value::Bool(_) => Value::String(String::from("Bool")),
        Value::Number(_) => Value::String(String::from("Number")),
        Value::String(_) => Value::String(String::from("String")),
        Value::Array(values) => Value::Array(values.iter().map(|v| hide(v)).collect()),
        Value::Object(values) => Value::Object(
            values
                .iter()
                .map(|(k, v)| (k.to_owned(), hide(v)))
                .collect(),
        ),
    };
    new_value
}
