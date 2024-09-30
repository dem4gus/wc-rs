use std::path::PathBuf;

use clap::Parser;
use wc_rs::{bytes, lines};

#[derive(Parser)]
struct Cli {
    file: PathBuf,

    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// print the newline counts
    #[arg(short, long)]
    lines: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut count: usize = 0;
    let content = std::fs::read_to_string(cli.file.as_os_str()).unwrap();

    if cli.bytes {
        count = bytes::count(content.as_str());
    }

    if cli.lines {
        count = lines::count(content.as_str());
    }

    println!(
        "{} {}",
        count,
        cli.file.file_name().unwrap().to_str().unwrap()
    );
}
