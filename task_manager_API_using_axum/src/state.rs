use std::sync::Arc;
use tokio::sync::Mutex;

use tracing::debug;

use crate::models::{auth::User, task::Task};

pub type UserStore = Arc<Mutex<Vec<User>>>;
pub type TaskStore = Arc<Mutex<Vec<Task>>>;

#[derive(Clone)]
pub struct AppState {
    pub users: UserStore,
    pub tasks: TaskStore,
}

impl AppState {
    pub fn new() -> Self {
        debug!("Initializing application state");

        let state = Self {
            users: Arc::new(Mutex::new(Vec::new())),
            tasks: Arc::new(Mutex::new(Vec::new())),
        };

        debug!("Application state initialized");

        state
    }
}
