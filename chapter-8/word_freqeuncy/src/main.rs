mod word_counter;

use std::fs;
use word_counter::WordCounter;

fn main() {
    match read_file_content() {
        Some(content) => {
            let mut word_counter = WordCounter::new(content);
            word_counter.process_content(3);
        }
        None => {
            eprintln!("Error, path not found, try again.");
        }
    }
}

fn read_file_content() -> Option<String> {
    match fs::read_to_string("file.txt") {
        Ok(file_content) => Some(file_content),
        Err(_) => None,
    }
}