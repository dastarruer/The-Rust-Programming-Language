pub struct WordCounter<'a> {
    text: &'a str,
}

impl<'a> WordCounter<'a> {
    fn count_words(&self) -> i32 {
        // Check if text is empty
        if self.text.is_empty() {
            return 0;
        }

        let mut counter = 1;
        for c in self.text.chars() {
            if c == ' ' {
                counter += 1;
            }
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_words() {
        let wc = WordCounter {
            text: "hello world",
        };
        assert_eq!(wc.count_words(), 2);
    }

    #[test]
    fn test_dashes() {
        let wc = WordCounter {
            text: "hello-world",
        };
        assert_eq!(wc.count_words(), 1);
    }

    #[test]
    fn test_empty_string() {
        let wc = WordCounter { text: "" };
        assert_eq!(wc.count_words(), 0);
    }
}
