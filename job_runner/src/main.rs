use async_trait::async_trait;
use std::{sync::Arc, thread, time::Duration};
use tokio::task::JoinHandle;
use trpl::Html;

pub trait Job: Send + Sync {
    fn name(&self) -> &str;
    fn is_cpu_bound(&self) -> bool;
}

#[async_trait]
pub trait AsyncJob {
    async fn run_async(&self) -> String;
}

pub trait BlockingJob {
    fn run_blocking(&self) -> String;
}

pub struct SleepJob {
    seconds: u64,
}

impl Job for SleepJob {
    fn name(&self) -> &str {
        "Sleep Job"
    }

    fn is_cpu_bound(&self) -> bool {
        false
    }
}

#[async_trait]
impl AsyncJob for SleepJob {
    async fn run_async(&self) -> String {
        trpl::sleep(Duration::from_secs(self.seconds)).await;
        format!("Slept for {}", self.seconds)
    }
}

pub struct SumJob {
    num: u32,
}

impl Job for SumJob {
    fn name(&self) -> &str {
        "Sum Job"
    }

    fn is_cpu_bound(&self) -> bool {
        true
    }
}

impl BlockingJob for SumJob {
    fn run_blocking(&self) -> String {
        let mut sum = 0;
        for i in 1..=self.num {
            sum += i;
        }
        format!("The Sum of the {} is {}", self.num, sum)
    }
}

pub struct FetchJob {
    url: String,
}

impl Job for FetchJob {
    fn name(&self) -> &str {
        "Fetch Job"
    }

    fn is_cpu_bound(&self) -> bool {
        false
    }
}

#[async_trait]
impl AsyncJob for FetchJob {
    async fn run_async(&self) -> String {
        let response = trpl::get(&self.url).await.text().await;
        Html::parse(&response)
            .select_first("title")
            .map(|title| title.inner_html())
            .expect("Failed to fetch title")
    }
}

pub struct JobRunner {
    async_handlers: Vec<JoinHandle<()>>,
    thread_handlers: Vec<thread::JoinHandle<()>>,
}

impl JobRunner {
    pub fn new() -> Self {
        Self {
            async_handlers: Vec::new(),
            thread_handlers: Vec::new(),
        }
    }

    pub fn submit_async_job<J>(&mut self, job: Arc<J>)
    where
        J: Job + AsyncJob + 'static,
    {
        let name = job.name().to_string();

        let handle = tokio::spawn(async move {
            let result = job.run_async().await;
            println!("[Async] {} finished: {}", name, result);
        });

        self.async_handlers.push(handle);
    }

    pub fn submit_blocking_job<J>(&mut self, job: Arc<J>)
    where
        J: Job + BlockingJob + 'static,
    {
        let name = job.name().to_string();

        let handle = thread::spawn(move || {
            let result = job.run_blocking();
            println!("[Blocking] {} finished: {}", name, result);
        });

        self.thread_handlers.push(handle);
    }
}

#[tokio::main]
async fn main() {
    let mut runner = JobRunner::new();

    let sleep_job = Arc::new(SleepJob { seconds: 2 });
    let sum_job = Arc::new(SumJob { num: 10 });
    let fetch_job = Arc::new(FetchJob {
        url: "https://axna.vercel.app/".to_string(),
    });

    runner.submit_async_job(sleep_job);
    runner.submit_async_job(fetch_job);
    runner.submit_blocking_job(sum_job);

    for handle in runner.async_handlers {
        handle.await.unwrap();
    }

    for handle in runner.thread_handlers {
        handle.join().unwrap();
    }

    println!("Job execution completed");
}
