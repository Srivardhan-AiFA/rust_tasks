use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    pub owner: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddTask {
    pub title: String,
    pub completed: bool,
}
