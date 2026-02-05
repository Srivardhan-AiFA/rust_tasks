use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct AddTodo {
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct DeleteTodo {
    pub id: u32,
}
