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
    /// add keywords to hide in the JSON
    #[arg(long = "add-words")]
    pub add_words: Vec<String>,
    /// remove keywords from hiding in the JSON
    #[arg(long = "remove-words")]
    pub remove_words: Vec<String>,
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn verify_args() {
        use clap::CommandFactory;
        HideArgs::command().debug_assert();
    }
}
