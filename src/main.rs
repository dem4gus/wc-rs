use std::path::PathBuf;

use clap::{Args, Parser};
use wc_rs::*;

#[derive(Debug, Args)]
struct CountMode {
    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    /// print the newline counts
    #[arg(short, long)]
    lines: bool,

    /// print the word counts
    #[arg(short, long)]
    words: bool,
}

#[derive(Debug, Parser)]
struct Cli {
    file: PathBuf,

    #[command(flatten)]
    count_mode: CountMode,
}

fn main() {
    let args = Cli::parse();

    let mut count: usize = 0;
    let content = std::fs::read_to_string(args.file.as_os_str()).unwrap();

    if args.count_mode.bytes {
        count = bytes::count(content.as_str());
    }

    if args.count_mode.lines {
        count = lines::count(content.as_str());
    }

    if args.count_mode.words {
        count = words::count(content.as_str());
    }

    println!(
        "{} {}",
        count,
        args.file.file_name().unwrap().to_str().unwrap()
    );
}
