use std::io;
use std::io::Write;

// Function to prompt user and read input
pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout()
        .flush()
        .expect("Error flushing stdout, please try again.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input, please try again.");

    input.trim().to_string()
}