use std::io;
use std::io::Write;

use crate::task_manager::TaskManager; // ✅ Import from crate root
use crate::Command;

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

// Function to get the user's command
pub fn get_command() {
    let mut task_manager = TaskManager::new("./tasks.json".to_string());

    // Get user command
    let command = match get_input(
        "Commands:\n1) Add task\n2) Edit task\n3) Complete task\n4) List tasks\n: ",
    )
    .parse::<u8>() // Parse as integer
    .ok()
    {
        Some(1) => Some(Command::Add),
        Some(2) => Some(Command::Edit),
        Some(3) => Some(Command::Complete),
        Some(4) => Some(Command::List),
        _ => None,
    };

    match command {
        Some(Command::Add) => task_manager.add_task(),
        Some(Command::List) => task_manager.list_tasks(),
        Some(Command::Complete) => task_manager.complete_task(),
        Some(Command::Edit) => task_manager.edit_tasks(),
        None => eprintln!("Error, invalid command. Please try again"),
    }
}
