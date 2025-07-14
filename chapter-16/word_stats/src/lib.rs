pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    /// Return the stats of a given &str
    fn get_word_stats(&self) -> String {
        let word_count = TextAnalyzer::get_word_count(self.text);
        let longest_word = TextAnalyzer::get_longest_word(self.text);

        format!(
            r"Stats:
        Word count: {}
        Longest word: {}",
            word_count, longest_word
        )
    }

    /// Strip non alphabetic chars and convert all chars to lowercase
    fn process_text(text: &str) -> String {
        let unwanted_chars = r#"!@#$%^&*()=_+`~,./;#'[]{}:"<>?"#;
        // Convert text to lowercase
        let mut new_text = text.to_lowercase();

        // Remove unwanted chars
        new_text.retain(|c| !unwanted_chars.contains(c));

        new_text
    }

    /// Return the word count of a given &str
    fn get_word_count(text: &str) -> usize {
        // Check if text is empty
        if text.is_empty() {
            return 0;
        }

        text.split_whitespace().collect::<Vec<&str>>().len()
    }

    /// Return the longest word of a given &str
    fn get_longest_word(text: &str) -> &str {
        let mut longest_word: &str = "";
        for word in text.split_whitespace().collect::<Vec<&str>>() {
            if word.len() > longest_word.len() {
                longest_word = word;
            }
        }
        longest_word
    }
}

#[cfg(test)]
mod tests {

    mod word_stats {
        use super::super::*;

        #[test]
        fn test_word_stats() {
            let text = TextAnalyzer {
                text: "Hello worlds",
            };

            assert_eq!(
                text.get_word_stats(),
                r"Stats:
        Word count: 2
        Longest word: worlds"
            )
        }
    }

    mod process_text {
        use super::super::*;

        #[test]
        fn test_process_text() {
            assert_eq!(
                TextAnalyzer::process_text("Hello, world!"),
                String::from("hello world")
            );

            assert_eq!(
                TextAnalyzer::process_text("hello-world"),
                String::from("hello-world")
            );
        }
    }

    mod word_count {
        use super::super::*;

        #[test]
        fn test_get_word_count() {
            assert_eq!(TextAnalyzer::get_word_count("hello world"), 2);
        }

        #[test]
        fn test_dashes() {
            assert_eq!(TextAnalyzer::get_word_count("hello-world"), 1);
        }

        #[test]
        fn test_empty_string() {
            assert_eq!(TextAnalyzer::get_word_count(""), 0);
        }
    }

    mod longest_word {
        use super::super::*;

        #[test]
        fn test_longest_word() {
            assert_eq!(TextAnalyzer::get_longest_word("hello-world"), "hello-world");

            assert_eq!(
                TextAnalyzer::get_longest_word(
                    "catastrophe hello there when alskdfjalsdkfjalksdfj"
                ),
                "alskdfjalsdkfjalksdfj"
            );
        }
    }
}
