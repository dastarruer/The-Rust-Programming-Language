pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
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
    use super::*;

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

    #[test]
    fn test_longest_word() {
        assert_eq!(TextAnalyzer::get_longest_word("hello-world"), "hello-world");

        assert_eq!(
            TextAnalyzer::get_longest_word("catastrophe hello there when alskdfjalsdkfjalksdfj"),
            "alskdfjalsdkfjalksdfj"
        );
    }
}
