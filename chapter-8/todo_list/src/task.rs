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
    // Can be an empty string if there is no description
    pub description: String,
    pub status: Status,
}
