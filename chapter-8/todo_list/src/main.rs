mod task;
mod task_manager;
mod utils;

use task_manager::TaskManager;
use utils::get_input;


enum Command {
    Add,
    Complete,
    Edit,
    List,
}

fn main() {
    let mut task_manager = TaskManager::new("./tasks.json".to_string());

    // Get user command
    let command = match get_input(
        "Commands:\n1) Add task\n2) Edit task\n3) Complete task\n4) List tasks\n: ",
    )
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
        Some(Command::Add) => task_manager.add_task(),
        Some(Command::List) => task_manager.list_tasks(),
        Some(Command::Complete) => task_manager.complete_task(),
        Some(Command::Edit) => task_manager.edit_tasks(),
        None => eprintln!("Error, invalid command. Please try again"),
    }
}
