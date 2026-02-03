use std::sync::{Arc, Mutex};

use tokio::task::JoinHandle;

use crate::{
    task::{Task, TaskResult},
    thread_pool::ThreadPool,
};

pub struct TaskScheduler {
    pool: ThreadPool,
    async_handlers: Mutex<Vec<JoinHandle<()>>>,
}

impl TaskScheduler {
    pub fn new(worker_count: usize) -> Self {
        Self {
            pool: ThreadPool::new(worker_count),
            async_handlers: Mutex::new(Vec::new()),
        }
    }

    pub fn submit(&self, task: Arc<dyn Task>) {
        let name = task.name().to_string();

        if let Some(async_task) = task.clone().as_async() {
            let handle = tokio::spawn(async move {
                let result = async_task.execute_async().await;
                match result {
                    TaskResult::Success(v) => {
                        println!("[Async] {} finished: {}", task.name(), v)
                    }
                    TaskResult::Failure(e) => {
                        println!("[Async] {} failed: {}", name, e)
                    }
                }
            });
            self.async_handlers.lock().unwrap().push(handle);
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

    pub async fn wait_for_async_to_complete(self) {
        for handle in self.async_handlers.into_inner().unwrap() {
            handle.await.unwrap();
        }
    }
}
