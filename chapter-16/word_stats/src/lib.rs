use std::sync::mpsc;

use std::thread;

pub struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    /// Return the stats of a given &str
    fn get_word_stats(&self) -> String {
        // Convert all letters to lowercase and remove punctuation
        let text = TextAnalyzer::process_text(self.text);

        // Each chunk will hold a part of the text
        let num_chunks = 4;

        // Clamp chunk size to be at least 1 to avoid errors
        let chunk_size = (text.len() / num_chunks).max(1);

        // Chunk the text into multiple Vec<String>'s
        let chunks: Vec<Vec<String>> = text
            .chunks(chunk_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let mut handles = Vec::new();

        // Channel for word count
        let (word_count_tx, word_count_rx) = mpsc::channel::<usize>();

        // Channel for longest word
        let (longest_word_tx, longest_word_rx) = mpsc::channel::<String>();

        for chunk in chunks {
            // Clone the transmitters so the originals do not get moved into the closure
            let word_count_tx = mpsc::Sender::clone(&word_count_tx);
            let longest_word_tx = mpsc::Sender::clone(&longest_word_tx);

            // Spawn a thread to handle each chunk
            let handle = thread::spawn(move || {
                let longest_word = TextAnalyzer::get_longest_word(chunk.clone());
                longest_word_tx.send(longest_word).unwrap();

                let word_count = TextAnalyzer::get_word_count(chunk);
                word_count_tx.send(word_count).unwrap();
            });

            handles.push(handle);
        }

        // Drop the transmitters so that the program does not hang
        drop(word_count_tx);
        drop(longest_word_tx);

        // Get the longest word
        let mut longest_word = String::new();
        for word in longest_word_rx {
            if word.len() > longest_word.len() {
                longest_word = word;
            }
        }

        // Add up the word count from each thread
        let mut total_word_count = 0;
        for word_count in word_count_rx {
            total_word_count += word_count;
        }

        // Wait until all threads have finished
        for handle in handles {
            handle.join().unwrap();
        }

        format!(
            r"Stats:
        Word count: {}
        Longest word: {}",
            total_word_count, longest_word
        )
    }

    /// Strip non alphabetic chars and convert all chars to lowercase
    fn process_text(text: &str) -> Vec<String> {
        let unwanted_chars = r#"!@#$%^&*()=_+`~,./;#'[]{}:"<>?"#;

        // Convert text to lowercase
        let mut new_text = text.to_lowercase();

        // Remove unwanted chars
        new_text.retain(|c| !unwanted_chars.contains(c));

        // Split text by whitespace for easier chunking later on
        new_text.split_whitespace().map(String::from).collect()
    }

    /// Return the word count of a given &str
    fn get_word_count(text: Vec<String>) -> usize {
        // The length of the vector will equal the number of words
        text.len()
    }

    /// Return the longest word of a given &str
    fn get_longest_word(text: Vec<String>) -> String {
        let mut longest_word = String::new();

        for word in text {
            if word.len() > longest_word.len() {
                longest_word = word;
            }
        }

        longest_word.to_string()
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
            );

            let text = TextAnalyzer { text: "Hello worl" };

            assert_eq!(
                text.get_word_stats(),
                r"Stats:
        Word count: 2
        Longest word: hello"
            );

            let text = TextAnalyzer {
                text: "Hello world!",
            };

            assert_eq!(
                text.get_word_stats(),
                r"Stats:
        Word count: 2
        Longest word: hello"
            );
        }
    }

    mod process_text {
        use super::super::*;

        #[test]
        fn test_process_text() {
            assert_eq!(
                TextAnalyzer::process_text("Hello, world!"),
                vec!["hello".to_string(), "world".to_string()]
            );

            assert_eq!(
                TextAnalyzer::process_text("hello-world"),
                vec!["hello-world".to_string()]
            );
        }
    }

    mod word_count {
        use super::super::*;

        #[test]
        fn test_get_word_count() {
            assert_eq!(
                TextAnalyzer::get_word_count(vec!["hello".to_string(), "world".to_string()]),
                2
            );
        }

        #[test]
        fn test_dashes() {
            let input = vec!["hello-world".to_string()];
            assert_eq!(TextAnalyzer::get_word_count(input), 1);
        }

        #[test]
        fn test_empty_string() {
            let input: Vec<String> = vec![];
            assert_eq!(TextAnalyzer::get_word_count(input), 0);
        }
    }
    mod longest_word {
        use super::super::*;

        #[test]
        fn test_longest_word() {
            let input = vec!["hello-world".to_string()];
            assert_eq!(
                TextAnalyzer::get_longest_word(input),
                "hello-world".to_string()
            );

            let input = vec![
                "catastrophe".to_string(),
                "hello".to_string(),
                "there".to_string(),
                "when".to_string(),
                "alskdfjalsdkfjalksdfj".to_string(),
            ];
            assert_eq!(
                TextAnalyzer::get_longest_word(input),
                "alskdfjalsdkfjalksdfj".to_string()
            );
        }
    }
}
