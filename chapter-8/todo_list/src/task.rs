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
    // Can be None
    pub description: Option<String>,
    pub status: Status,
}
