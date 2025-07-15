use std::fs::File;
use std::io::{self, BufRead, BufReader};

use word_stats::TextAnalyzer;

fn main() -> io::Result<()> {
    let file = File::open("test.txt")?;
    let reader = BufReader::new(file);
    let mut content = String::new();

    for line in reader.lines() {
        let line = line?; // Unwrap Result<String>
        // Append the current line to content
        content = format!("{}{}", content, line);
    }

    let text_analyzer = TextAnalyzer { text: &content };
    println!("{}", text_analyzer.get_word_stats());

    Ok(())
}
