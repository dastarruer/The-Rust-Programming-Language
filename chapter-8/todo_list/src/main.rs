use std::io;
use std::io::Write;

enum Command {
    Add,
    Complete,
    List,
}

fn main() {
    print!("Commands:\n1) Add task\n2) Mark task completed\n3) List tasks\n: ");
    std::io::stdout().flush().expect("Error flushing stdout, please try again.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input, please try again.");
    
    let choice = match input.trim() {
        "1" => Some(Command::Add),
        "2" => Some(Command::Complete),
        "3" => Some(Command::List),
        _ => None,
    };
}
