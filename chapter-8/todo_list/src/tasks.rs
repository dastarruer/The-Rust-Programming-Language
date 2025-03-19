// Shows if a task is complete or incomplete
pub enum Status {
    Incomplete,
    Complete,
}

pub struct Task {
    pub name: String,
    // Can be None
    pub description: Option<String>,
    pub status: Status,
}
