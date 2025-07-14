pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn count_words(&self) -> usize {
        // Check if text is empty
        if self.text.is_empty() {
            return 0;
        }

        self.text.split_whitespace().collect::<Vec<_>>().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let wc = TextAnalyzer {
            text: "hello world",
        };
        assert_eq!(wc.count_words(), 2);
    }

    #[test]
    fn test_dashes() {
        let wc = TextAnalyzer {
            text: "hello-world",
        };
        assert_eq!(wc.count_words(), 1);
    }

    #[test]
    fn test_empty_string() {
        let wc = TextAnalyzer { text: "" };
        assert_eq!(wc.count_words(), 0);
    }
}
