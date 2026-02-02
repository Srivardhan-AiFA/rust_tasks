use async_trait::async_trait;
use std::{sync::Arc, time::Duration};

use crate::task::{AsyncExecutable, Task, TaskResult};

pub struct SleepTask {
    pub seconds: u64,
}

impl Task for SleepTask {
    fn name(&self) -> &'static str {
        "SleepTask"
    }

    fn as_async(self: Arc<Self>) -> Option<Arc<dyn AsyncExecutable>> {
        Some(self)
    }
}

#[async_trait]
impl AsyncExecutable for SleepTask {
    async fn execute_async(&self) -> TaskResult {
        tokio::time::sleep(Duration::from_secs(self.seconds)).await;
        TaskResult::Success(format!("Slept for {} seconds", self.seconds))
    }
}
