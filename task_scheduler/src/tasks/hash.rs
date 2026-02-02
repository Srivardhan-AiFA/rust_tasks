use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::task::{BlockingExecutable, Task, TaskResult};

pub struct HashTask {
    pub input: String,
}

impl Task for HashTask {
    fn name(&self) -> &'static str {
        "HashTask"
    }

    fn as_blocking(self: Arc<Self>) -> Option<Arc<dyn BlockingExecutable>> {
        Some(self)
    }
}

impl BlockingExecutable for HashTask {
    fn execute_blocking(&self) -> TaskResult {
        let hash = Sha256::digest(self.input.as_bytes());
        TaskResult::Success(format!("{:x}", hash))
    }
}
