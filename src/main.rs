use std::io;

use clap::{Args, Parser};
use wc_rs::counter::{CountResult, Mode};

#[derive(Debug, Args)]
struct CountMode {
    /// print the byte counts
    #[arg(short = 'c', long)]
    bytes: bool,

    // print the character counts
    #[arg(short = 'm', long)]
    chars: bool,

    /// print the newline counts
    #[arg(short, long)]
    lines: bool,

    /// print the word counts
    #[arg(short, long)]
    words: bool,
}

#[derive(Debug, Parser)]
struct Cli {
    file: Option<String>,

    #[command(flatten)]
    count_mode: CountMode,
}

const DEFAULT_MODES: [Mode; 3] = [Mode::Bytes, Mode::Lines, Mode::Words];

fn main() {
    let args = Cli::parse();

    let mut op_modes = Vec::new();
    if args.count_mode.bytes {
        op_modes.push(Mode::Bytes);
    }
    if args.count_mode.chars {
        op_modes.push(Mode::Chars);
    }
    if args.count_mode.lines {
        op_modes.push(Mode::Lines);
    }
    if args.count_mode.words {
        op_modes.push(Mode::Words);
    }
    if op_modes.is_empty() {
        op_modes = DEFAULT_MODES.to_vec();
    }

    let content = match args.file {
        Some(ref filename) => std::fs::read_to_string(filename).unwrap(),
        None => io::read_to_string(io::stdin()).unwrap(),
    };

    let counter = CountResult::new(
        &content,
        args.file.unwrap_or(String::from("")).as_str(),
        op_modes,
    );

    println!("{}", counter);
}
