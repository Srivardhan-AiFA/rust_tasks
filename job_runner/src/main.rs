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

// Sleep Job
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

// Sum Job
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
        for i in 0..self.num {
            sum += i + 1
        }
        format!("The Sum of the {} is {sum}", self.num)
    }
}

// Fetch Job
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
            .expect("Error while fetching the URL's title")
    }
}

pub struct JobRunner {
    async_handlers: Vec<JoinHandle<()>>,
    thread_handlers: Vec<thread::JoinHandle<()>>,
}

impl JobRunner {
    fn new() -> Self {
        Self {
            async_handlers: Vec::new(),
            thread_handlers: Vec::new(),
        }
    }

    fn submit_async_jobs<J>(&mut self, job: Arc<J>)
    where
        J: Job + AsyncJob + Send + Sync + 'static,
    {
        let name = job.name().to_string();

        let handle = tokio::spawn(async move {
            let result = job.run_async().await;
            println!("[Async] {} finished: {}", name, result);
        });
        self.async_handlers.push(handle);
    }

    fn submit_thread_jobs<J>(&mut self, job: Arc<J>)
    where
        J: Job + BlockingJob + Send + Sync + 'static,
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

    runner.submit_async_jobs(sleep_job);
    runner.submit_async_jobs(fetch_job);
    runner.submit_thread_jobs(sum_job);

    for i in runner.async_handlers {
        i.await.unwrap();
    }

    for j in runner.thread_handlers {
        j.join().unwrap()
    }

    println!("Job(s) execuation is complected");
}
