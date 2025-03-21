use std::io;
use std::io::Write;

mod task;
mod task_manager;
use task::{Status, Task};
use task_manager::TaskManager;

enum Command {
    Add,
    Complete,
    Edit,
    List,
}

fn main() {
    let mut task_manager = TaskManager::new("./tasks.json".to_string());

    // Get user command
    let command = match get_input("Commands:\n1) Add task\n2) Edit task\n3) Complete task\n4) List tasks\n: ")
        .parse::<u8>() // Cast to a uint so that the compiler doesn't get mad about comparing String to &str
        .ok()
    {
        Some(1) => Some(Command::Add),
        Some(2) => Some(Command::Edit),
        Some(3) => Some(Command::Complete),
        Some(4) => Some(Command::List),
        _ => None,
    };

    match command {
        Some(Command::Add) => {
            let name = get_input("Task name: ");

            let description = get_input("Task description (press enter to leave empty): ");

            let task = Task::new(name, description);

            task_manager.add_task(task);
        }
        Some(Command::List) => {
            task_manager.list_tasks();
        },
        // TODO: Implement completing and editing tasks
        Some(Command::Complete) => todo!(),
        Some(Command::Edit) => todo!(),
        None => eprintln!("Error, invalid command. Please try again"),
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
