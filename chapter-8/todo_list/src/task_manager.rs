use std::fs;

pub struct TaskManager {
    // The file to which JSON data will be stored
    filename: String,

    // Stores the tasks JSON data
    pub tasks: String,
}

impl TaskManager {
    pub fn new(filename: String) -> Self {
        let mut task_manager = TaskManager {
            filename,
            // Initialize as empty first
            tasks: "".to_string(),
        };
        task_manager.load_tasks();
        task_manager
    }

    // Initialize tasks.JSON with an empty JSON object
    fn init_tasks_file(&self) -> String {
        println!("Tasks not found, initializing...");

        let json = "{}";
        fs::write(&self.filename, &json).expect("Error, railed to create file");

        json.to_string()
    }
    
    // Read tasks.json into self.tasks
    fn load_tasks(&mut self) {
        self.tasks = match fs::read_to_string(&self.filename) {
            Ok(data) => {
                match serde_json::from_str::<serde_json::Value>(&data) {
                    Ok(_) => data,
                    Err(_) => {
                        eprintln!("Unable to load data, file may be corrupt.");
                        "{}".to_string() // Return default empty JSON
                    }
                }
            }
            // If an error is thrown because there is no tasks.json file, initialize tasks.json
            Err(_) => self.init_tasks_file()
        };
    }
}
