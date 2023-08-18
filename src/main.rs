use clap::Parser;

use crate::hide_args::HideArgs;

mod hide_args;
mod words_storage;

fn main() {
    let args = HideArgs::parse();

    println!("args received: {:?}", args);
}
