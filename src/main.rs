use clap::Parser;

use crate::hide_args::HideArgs;

mod error;
mod hide_args;

fn main() {
    let args = HideArgs::parse();

    println!("args received: {:?}", args);
}
