use async_trait::async_trait;
use std::sync::Arc;

use crate::task::{AsyncExecutable, Task, TaskResult};

pub struct HttpFetchTask {
    pub url: String,
}

impl Task for HttpFetchTask {
    fn name(&self) -> &'static str {
        "HttpFetchTask"
    }

    fn as_async(self: Arc<Self>) -> Option<Arc<dyn AsyncExecutable>> {
        Some(self)
    }
}

#[async_trait]
impl AsyncExecutable for HttpFetchTask {
    async fn execute_async(&self) -> TaskResult {
        let response = match reqwest::get(&self.url).await {
            Ok(r) => r,
            Err(e) => return TaskResult::Failure(e.to_string()),
        };

        match response.text().await {
            Ok(body) => TaskResult::Success(format!("Fetched {} bytes", body.len())),
            Err(e) => TaskResult::Failure(e.to_string()),
        }
    }
}
