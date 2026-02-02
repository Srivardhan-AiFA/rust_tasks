use async_trait::async_trait;
use std::sync::Arc;

#[derive(Debug)]
pub enum TaskResult {
    Success(String),
    Failure(String),
}

pub trait Task: Send + Sync {
    fn name(&self) -> &'static str;

    fn as_async(self: Arc<Self>) -> Option<Arc<dyn AsyncExecutable>> {
        None
    }

    fn as_blocking(self: Arc<Self>) -> Option<Arc<dyn BlockingExecutable>> {
        None
    }
}

#[async_trait]
pub trait AsyncExecutable: Send + Sync {
    async fn execute_async(&self) -> TaskResult;
}

pub trait BlockingExecutable: Send + Sync {
    fn execute_blocking(&self) -> TaskResult;
}
