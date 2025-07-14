pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    fn get_word_count(&self) -> usize {
        // Check if text is empty
        if self.text.is_empty() {
            return 0;
        }

        self.text.split_whitespace().collect::<Vec<&str>>().len()
    }

    fn get_longest_word(&self) -> &str {
        let mut longest_word: &str = "";
        for word in self.text.split_whitespace().collect::<Vec<&str>>() {
            if word.len() > longest_word.len() {
                longest_word = word;
            }
        }
        longest_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_word_count() {
        let wc = TextAnalyzer {
            text: "hello world",
        };
        assert_eq!(wc.get_word_count(), 2);
    }

    #[test]
    fn test_dashes() {
        let wc = TextAnalyzer {
            text: "hello-world",
        };
        assert_eq!(wc.get_word_count(), 1);
    }

    #[test]
    fn test_empty_string() {
        let wc = TextAnalyzer { text: "" };
        assert_eq!(wc.get_word_count(), 0);
    }

    #[test]
    fn test_longest_word() {
        let wc = TextAnalyzer {
            text: "hello-world",
        };
        assert_eq!(wc.get_longest_word(), "hello-world");

        let wc = TextAnalyzer {
            text: "catastrophe hello there when alskdfjalsdkfjalksdfj",
        };
        assert_eq!(wc.get_longest_word(), "alskdfjalsdkfjalksdfj");
    }
}
