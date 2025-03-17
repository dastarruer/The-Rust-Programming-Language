use std::collections::HashMap;

pub struct WordCounter {
    pub content: String,
    pub words: HashMap<String, u64>,
    pub total_word_count: u64,
}

impl WordCounter {
    pub fn new(content: String) -> Self {
        WordCounter {
            content,
            words: HashMap::new(),
            total_word_count: 0,
        }
    }

    fn is_char_valid(&self, word: &String, ch: char) -> bool {
        // Chars that are allowed, such as ' (eg. "he's", "there's"), and - (eg. "fruit-like") 
        let allowed_non_alphabetical_chars = ['\'', '-'];

        ch.is_alphabetic()
                || (allowed_non_alphabetical_chars.contains(&ch) && !word.is_empty())
    }

    fn process_word(&mut self, word: &mut String) {
        // If the word does not contain non-alphabetic chars ie. ',', '-', etc.
        if word.chars().any(|c| c.is_alphabetic()) {
            self.total_word_count += 1; // Increase total word count

            // Get current count or default to 0
            let word_count = self.words.get(word).copied().unwrap_or(0);

            // Update word count
            self.words.insert(word.clone(), word_count + 1);
        }
        // Reset `word` for the next word
        word.clear();
    }

    // Count words and update the hash map
    fn count_words(&mut self) {
        let mut word = String::new();
        let chars: Vec<char> = self.content.chars().collect();


        for ch in chars {
            // If the character is alphabetic, or it is an allowed char, and is not empty
            if self.is_char_valid(&word, ch)
            {
                word.push(ch);
            } else {
                self.process_word(&mut word);
            }
        }

        // Ensure the last word is processed if content doesn't end with punctuation
        self.process_word(&mut word);
    }

    fn get_top_n_words(&self, n: usize) -> Vec<(String, u64)> {
        // Convert the hash map to a vector of tuples, each containing the word and its count
        let mut words: Vec<(String, u64)> = self
            .words
            .iter()
            .map(|(word, count)| (word.clone(), *count))
            .collect();

        // Remove common words so we get more interesting results
        let excluded_words = [
            "the", "and", "of", "is", "a", "in", "to", "he", "his", "that", "was", "at", "as",
            "her", "him",
        ];
        words.retain(|(word, _)| !excluded_words.contains(&word.as_str()));

        // Sort by count in descending order
        words.sort_by(|a, b| b.1.cmp(&a.1));

        // Take top n words
        words.into_iter().take(n).collect()
    }

    pub fn print_top_n_words(&self, n: usize) {
        let words = self.get_top_n_words(n);

        // Print total word count and table header
        println!("Total Word Count: {}\n", self.total_word_count);
        println!("Top words:"); // Header

        // Table border and headers
        println!("+----+---------------+----------+");
        println!("| {:<2} | {:<13} | {:<8} |", "#", "Word", "Count");
        println!("+----+---------------+----------+");

        // Print top words
        for (i, (word, count)) in words.iter().enumerate() {
            println!("| {:<2} | {:<13} | {:<8} |", i + 1, word, count);
        }

        println!("+----+---------------+----------+"); // Closing border
    }

    pub fn process_content(&mut self, n: usize) {
        self.count_words();
        self.print_top_n_words(n);
    }
}
