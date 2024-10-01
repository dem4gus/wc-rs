pub fn count(text: &str) -> u64 {
    text.split_ascii_whitespace().count() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_single_word() {
        let input = "word\n";
        let want = 1;
        let got = count(input);
        assert_eq!(want, got);
    }

    #[test]
    fn counts_multiple_words() {
        let input = "this line has several words\n";
        let want = 5;
        let got = count(input);
        assert_eq!(want, got);
    }

    #[test]
    fn counts_across_newline_characters() {
        let input = "this string has several words\nsplit across several\ndifferent lines\n";
        let want = 10;
        let got = count(input);
        assert_eq!(want, got);
    }
}
