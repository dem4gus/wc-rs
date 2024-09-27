fn count_bytes(text: &str) -> usize {
    text.len()
}

#[cfg(test)]
mod count_bytes {
    use super::*;

    #[test]
    fn succeeds() {
        let input = "this is a test string";
        let want = 21;
        let got = count_bytes(input);
        assert_eq!(got, want);
    }

    #[test]
    fn returns_zero_for_empty_input() {
        let input = "";
        let want = 0;
        let got = count_bytes(input);
        assert_eq!(got, want);
    }

    #[test]
    fn correctly_counts_multibyte_characters() {
        let input = "孫子";
        let want = 6;
        let got = count_bytes(input);
        assert_eq!(got, want);
    }
}
