pub fn count(text: &str) -> usize {
    return if text.ends_with('\n') {
        text.lines().count()
    } else {
        // don't count unterminated lines
        text.lines().count() - 1
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lf_line_endings() {
        let input = "single newline character\n";
        let want = 1;
        let got = count(input);
        assert_eq!(want, got);
    }

    #[test]
    fn no_newline() {
        let input = "no newline";
        let want = 0;
        let got = count(input);
        assert_eq!(want, got);
    }

    #[test]
    fn crlf_line_endings() {
        let input = "chunk of text\r\nwith CRLF\r\nline endings\r\n";
        let want = 3;
        let got = count(input);
        assert_eq!(want, got);
    }

    #[test]
    fn cr_endings_only() {
        let input = "this test\ronly has\rcarriage returns\r";
        let want = 0;
        let got = count(input);
        assert_eq!(want, got);
    }
}
