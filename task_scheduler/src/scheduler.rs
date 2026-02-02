use std::sync::Arc;

use crate::{
    task::{Task, TaskResult},
    thread_pool::ThreadPool,
};

pub struct TaskScheduler {
    pool: ThreadPool,
}

impl TaskScheduler {
    pub fn new(worker_count: usize) -> Self {
        Self {
            pool: ThreadPool::new(worker_count),
        }
    }

    pub fn submit(&self, task: Arc<dyn Task>) {
        let name = task.name().to_string();

        if let Some(async_task) = task.clone().as_async() {
            tokio::spawn(async move {
                match async_task.execute_async().await {
                    TaskResult::Success(v) => {
                        println!("[Async] {} finished: {}", name, v)
                    }
                    TaskResult::Failure(e) => {
                        println!("[Async] {} failed: {}", name, e)
                    }
                }
            });
        } else if let Some(blocking_task) = task.clone().as_blocking() {
            self.pool
                .execute(move || match blocking_task.execute_blocking() {
                    TaskResult::Success(v) => {
                        println!("[Blocking] {} finished: {}", name, v)
                    }
                    TaskResult::Failure(e) => {
                        println!("[Blocking] {} failed: {}", name, e)
                    }
                });
        }
    }
}
