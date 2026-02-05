use std::sync::Arc;
use tokio::sync::Mutex;

use crate::models::todo::Todo;

pub type Db = Arc<Mutex<Vec<Todo>>>;
