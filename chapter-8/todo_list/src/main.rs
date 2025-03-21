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
    let choice = match get_input("Commands:\n1) Add task\n2) Complete task\n3) List tasks\n: ")
        .parse::<u8>()
        .ok()
    {
        Some(1) => Some(Command::Add),
        Some(2) => Some(Command::Complete),
        Some(3) => Some(Command::List),
        _ => None,
    };

    match choice {
        Some(Command::Add) => {
            let name = get_input("Task name: ");

            let description = get_input("Task description (press enter to leave empty): ");

            let task = Task {
                name: name,
                description: description,
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

// Function to prompt user and read input
fn get_input(prompt: &str) -> String {
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
