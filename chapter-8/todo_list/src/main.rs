mod task;
mod task_manager;
mod utils;

use utils::get_command;

enum Command {
    Add,
    Complete,
    Edit,
    List,
}

fn main() {
    loop {
        get_command();
    }
}
