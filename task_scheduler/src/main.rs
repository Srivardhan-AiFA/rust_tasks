mod scheduler;
mod task;
mod tasks;
mod thread_pool;

use std::sync::Arc;

use scheduler::TaskScheduler;
use tasks::{hash::HashTask, http_fetch::HttpFetchTask, sleep::SleepTask};

#[tokio::main]
async fn main() {
    let scheduler = TaskScheduler::new(4);

    scheduler.submit(Arc::new(SleepTask { seconds: 2 }));
    scheduler.submit(Arc::new(HttpFetchTask {
        url: "https://axna.vercel.app".to_string(),
    }));
    scheduler.submit(Arc::new(HashTask {
        input: "hello".to_string(),
    }));
    scheduler.submit(Arc::new(HashTask {
        input: "rust".to_string(),
    }));
    scheduler.wait_for_async_to_complete().await
}
