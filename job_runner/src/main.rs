use async_trait::async_trait;
use std::{sync::Arc, thread, time::Duration};
use tokio::task::JoinHandle;
use trpl::Html;

pub trait Job: Send + Sync {
    fn name(&self) -> &str;
    fn as_async(self: Arc<Self>) -> Option<Arc<dyn AsyncJob>> {
        None
    }
    fn as_blocking(self: Arc<Self>) -> Option<Arc<dyn BlockingJob>> {
        None
    }
}

#[async_trait]
pub trait AsyncJob: Send + Sync {
    async fn run_async(&self) -> String;
}

pub trait BlockingJob: Send + Sync {
    fn run_blocking(&self) -> String;
}

pub struct SleepJob {
    seconds: u64,
}

impl Job for SleepJob {
    fn name(&self) -> &str {
        "Sleep Job"
    }

    fn as_async(self: Arc<Self>) -> Option<Arc<dyn AsyncJob>> {
        Some(self)
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

    fn as_blocking(self: Arc<Self>) -> Option<Arc<dyn BlockingJob>> {
        Some(self)
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

    fn as_async(self: Arc<Self>) -> Option<Arc<dyn AsyncJob>> {
        Some(self)
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

enum JobHandler {
    Async(JoinHandle<()>),
    Blocking(thread::JoinHandle<()>),
}

pub struct JobRunner {
    handlers: Vec<JobHandler>,
}

impl JobRunner {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn submit(&mut self, job: Arc<dyn Job>) {
        let name = job.name().to_string();

        if let Some(blocking_job) = job.clone().as_blocking() {
            let handle = thread::spawn(move || {
                let result = blocking_job.run_blocking();
                println!("[Blocking] {} finished: {}", name, result);
            });

            self.handlers.push(JobHandler::Blocking(handle));
        } else if let Some(async_job) = job.clone().as_async() {
            let handle = tokio::spawn(async move {
                let result = async_job.run_async().await;
                println!("[Async] {} finished: {}", name, result);
            });

            self.handlers.push(JobHandler::Async(handle));
        }
    }

    pub async fn run(self) {
        for handle in self.handlers {
            match handle {
                JobHandler::Async(ha) => {
                    ha.await.unwrap();
                }
                JobHandler::Blocking(hb) => {
                    hb.join().unwrap();
                }
            }
        }
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

    runner.submit(sleep_job);
    runner.submit(fetch_job);
    runner.submit(sum_job);

    runner.run().await;

    println!("Job execution completed");
}
