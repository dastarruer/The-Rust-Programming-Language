use std::fs;

use crate::task::Task;

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
        fs::write(&self.filename, &json).expect("Error, railed to create file");

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

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
        self.update_tasks();
    }

    // List all of the user's tasks
    pub fn list_tasks(&self) {
        for task in &self.tasks  {
            task.print_task();
        }
    }
}
