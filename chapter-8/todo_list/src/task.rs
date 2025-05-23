use serde_derive::Deserialize;
use serde_derive::Serialize;

// Shows if a task is complete or incomplete
#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Incomplete,
    Complete,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
    // Some tasks will not have a description
    pub description: Option<String>,
    pub status: Status,
}

impl Task {
    pub fn new(name: String, description: Option<String>) -> Self {
        Task {
            name,
            description,
            status: Status::Incomplete,
        }
    }

    pub fn print_task(&self) {
        if let Some(desc) = &self.description {
            println!("[{:?}] {} - {}", self.status, self.name, desc);
        } else {
            println!("[{:?}] {}", self.status, self.name);
        }
    }
}
