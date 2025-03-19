use std::io;
use std::io::Write;

mod tasks;
use tasks::Task;

enum Command {
    Add,
    Complete,
    List,
}

fn main() {
    let choice = match read_user_input().trim() {
        "1" => Some(Command::Add),
        "2" => Some(Command::Complete),
        "3" => Some(Command::List),
        _ => None,
    };

    match choice {
        None => println!("Error, invalid command. Please try again"),
        _ => {
            // TODO: Implement adding, completing and listing tasks
            todo!();
        }
    }
}

// Print the commands and read the user's input
fn read_user_input() -> String {
    // Print commands
    print!("Commands:\n1) Add task\n2) Mark task completed\n3) List tasks\n: ");
    std::io::stdout()
        .flush()
        .expect("Error flushing stdout, please try again.");

    // Read user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input, please try again.");

    input
}
