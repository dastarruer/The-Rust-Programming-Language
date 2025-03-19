pub struct TaskManager {
    // The file to which json data will be stored
    filename: String,
    // Stores the tasks json data
    tasks: String,
}

impl TaskManager {
    pub fn new(filename: String) -> Self {
        TaskManager {
            filename,
            tasks: "hello".to_string(),
        }
    }
}
