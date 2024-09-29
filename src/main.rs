use std::path::PathBuf;

use clap::Parser;
use wc_rs::bytes;

#[derive(Parser)]
struct Cli {
    file: PathBuf,

    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,
}
fn main() {
    let cli = Cli::parse();

    if cli.bytes {
        let content = std::fs::read_to_string(cli.file.as_os_str()).unwrap();
        let count = bytes::count(&content);
        println!(
            "{} {}",
            count,
            cli.file.file_name().unwrap().to_str().unwrap()
        );
    }
}
