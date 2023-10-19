use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
#[group(multiple = true)]
pub struct HideArgs {
    /// path to the input JSON file
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    pub input_file: Option<PathBuf>,
    /// path to the output file, requires input_file
    #[arg(
        short = 'o',
        long = "output",
        value_name = "FILE",
        requires = "input_file"
    )]
    pub output_file: Option<PathBuf>,
    /// enable debug mode
    #[arg(short, long)]
    pub debug: bool,
    /// add keys to hide in the JSON
    #[arg(long = "add-keys", value_delimiter = ',')]
    pub add_keys: Vec<String>,
    /// remove keys from hiding in the JSON
    #[arg(long = "remove-keys", value_delimiter = ',')]
    pub remove_keys: Vec<String>,
}
