use std::fs;

use crate::{
    task::{Status, Task},
    utils::get_input,
};

pub struct TaskManager {
    // The file to which JSON data will be stored
    filename: String,

    // Stores the tasks JSON data
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new(filename: String) -> Self {
        let mut task_manager = TaskManager {
            filename,
            // Initialize as empty first
            tasks: Vec::new(),
        };
        task_manager.load_tasks();
        task_manager
    }

    // Initialize self.filename with an empty JSON object
    fn init_tasks_file(&self) -> String {
        println!("Tasks not found, initializing...");

        let json = "{}";
        fs::write(&self.filename, json).expect("Error, railed to create file");

        json.to_string()
    }

    // Read self.filename into self.tasks
    fn load_tasks(&mut self) {
        self.tasks = match fs::read_to_string(&self.filename) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Task>>(&content) {
                    Ok(tasks) => tasks, // Successfully parsed tasks
                    Err(_) => {
                        eprintln!("Error, unable to load data; file may be corrupt.");
                        Vec::new() // Return an empty vector if JSON is invalid
                    }
                }
            }
            Err(_) => {
                eprintln!("Tasks file not found, creating new one...");
                self.init_tasks_file();
                Vec::new() // Return an empty vector when file is created
            }
        };
    }

    // Update self.filename with self.tasks
    fn update_tasks(&self) {
        // Convert self.tasks to JSON string
        let json = serde_json::to_string_pretty(&self.tasks)
            .expect("Error converting tasks to JSON, please try again.");

        // Write JSON string to self.filename
        fs::write(&self.filename, &json).expect("Error, railed to create file");
    }

    pub fn add_task(&mut self) {
        // Get the task name from the user
        let name = get_input("Task name: ");

        // Get the description from the user
        let description = get_input("Task description (press enter to leave empty): ");
        // Convert description to an Option
        let description = match description.is_empty() {
            true => None,
            false => Some(description),
        };

        let task = Task::new(name, description);

        self.tasks.push(task);
        self.update_tasks();
    }

    pub fn complete_task(&mut self) {
        // List all tasks to the user
        self.list_tasks();

        // Get the task number the user wants to complete
        let task = get_input("Input task to be completed: ");

        match task.parse::<usize>() {
            // Check if index is smaller than tasks.length to prevent goin gout of bounds
            Ok(index) if index - 1 < self.tasks.len() => {
                // Set the task to complete
                self.tasks[index - 1].status = Status::Complete;

                // Update filename.json
                self.update_tasks();
            }
            Ok(_) => println!("Invalid number, out of range."),
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }

    pub fn edit_tasks(&mut self) {
        // List all tasks to the user
        self.list_tasks();

        // Get the task number the user wants to complete
        let task = get_input("Input task to be edited: ");
        
        match task.parse::<usize>() {
            // Check if index is smaller than tasks.length to prevent going out of bounds
            Ok(index) if index - 1 < self.tasks.len() => {
                let task = self.tasks.get_mut(index - 1).unwrap();
                // Get new name and description from user
                let name = get_input("New task name (press enter to leave as is): ");
                let description = get_input("New task description (press enter to leave as is): ");

                // Set new name and description
                if !name.is_empty() {
                    task.name = name;
                }
                if !description.is_empty() {
                    task.description = Some(description);
                }

                // Update filename.json
                self.update_tasks();
            }
            Ok(_) => println!("Invalid number, out of range."),
            Err(_) => println!("Invalid input, please enter a number."),
        }
    }

    // List all of the user's tasks
    pub fn list_tasks(&self) {
        for (i, task) in self.tasks.iter().enumerate() {
            print!("#{} - ", i + 1);
            task.print_task();
        }
    }
}
