use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub sensitive_keys: Option<HashSet<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            sensitive_keys: Some(HashSet::new()),
        }
    }
}
