use std::{collections::HashMap, fmt::Display};

use crate::{bytes, lines, words};

#[derive(Eq, PartialEq, Debug, Hash)]
pub enum Mode {
    Bytes,
    Lines,
    Words,
}

#[derive(Debug)]
pub struct CountResult {
    totals: HashMap<Mode, u64>,
    filename: String,
}

impl CountResult {
    pub fn new(text: &str, filename: &str, modes: Vec<Mode>) -> CountResult {
        let mut totals = HashMap::new();

        for mode in modes {
            let total = match mode {
                Mode::Bytes => bytes::count(text),
                Mode::Lines => lines::count(text),
                Mode::Words => words::count(text),
            };
            totals.insert(mode, total);
        }
        CountResult {
            totals,
            filename: filename.to_owned(),
        }
    }
}

const REPORT_ORDER: [Mode; 3] = [Mode::Lines, Mode::Words, Mode::Bytes];

impl Display for CountResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatted_count = Vec::new();
        for mode in REPORT_ORDER {
            match self.totals.get(&mode) {
                Some(tot) => formatted_count.push(format!("{}", tot)),
                None => {}
            }
        }

        write!(f, "{} {}", formatted_count.join(" "), self.filename)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "this is a test string\n";
    const FILENAME: &str = "foo.txt";

    #[test]
    fn counts_only_bytes() {
        let want = "22 foo.txt";
        let got = format!("{}", CountResult::new(INPUT, FILENAME, vec![Mode::Bytes]));
        assert_eq!(want, got);
    }

    #[test]
    fn counts_only_lines() {
        let want = "1 foo.txt";
        let got = format!("{}", CountResult::new(INPUT, FILENAME, vec![Mode::Lines]));
        assert_eq!(want, got);
    }

    #[test]
    fn counts_only_words() {
        let want = "5 foo.txt";
        let got = format!("{}", CountResult::new(INPUT, FILENAME, vec![Mode::Words]));
        assert_eq!(want, got);
    }
}
