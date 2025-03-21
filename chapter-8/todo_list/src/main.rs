use std::io;
use std::io::Write;

mod task;
mod task_manager;
use task::{Status, Task};
use task_manager::TaskManager;

enum Command {
    Add,
    Complete,
    List,
}

fn main() {
    let mut task_manager = TaskManager::new("./tasks.json".to_string());
    let choice = match get_command().trim() {
        "1" => Some(Command::Add),
        "2" => Some(Command::Complete),
        "3" => Some(Command::List),
        _ => None,
    };

    match choice {
        Some(Command::Add) => {
            print!("Task name: ");
            std::io::stdout()
                .flush()
                .expect("Error flushing stdout, please try again.");
            let mut name = String::new();
            io::stdin()
                .read_line(&mut name)
                .expect("Error reading input, please try again.");

            print!("Task description: ");
            std::io::stdout()
                .flush()
                .expect("Error flushing stdout, please try again.");

            let mut description = String::new();
            io::stdin()
                .read_line(&mut description)
                .expect("Error reading input, please try again.");

            let task = Task {
                name: name.trim().to_string(),
                description: description.trim().to_string(),
                status: Status::Incomplete,
            };
            task_manager.add_task(task);
            println!("{:?}", task_manager.tasks);
        }
        None => eprintln!("Error, invalid command. Please try again"),
        _ => {
            // TODO: Implement completing, listing and editing tasks
            todo!();
        }
    }
}

// Print the commands and read the user's input
fn get_command() -> String {
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
