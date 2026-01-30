use async_trait::async_trait;
use std::{sync::Arc, thread, time::Duration};
use tokio::task::JoinHandle;
use trpl::Html;
pub trait Job: Send + Sync {
    fn name(&self) -> &str;
    fn is_cpu_bound(&self) -> bool;
}
// Copy from the assignment - Creating a trait Job which extend to Send and Sync (I don't know what those two do seeing them for the first time)
// It has two functions one is name that gives the name of the Job and the second function will tell weather it is a thread releted or async releted task

#[async_trait] // What the heck is this?
pub trait AsyncJob {
    async fn run_async(&self) -> String;
}
// Copy from the assignment - It has a async function declaration that returns a String (That string is what we need to print in the console)

pub trait BlockingJob {
    fn run_blocking(&self) -> String;
}
// Copy from the assignment - It has a function delaration which is also returns a String (That string is what we need to print in the console)

// IMPORTANT - I was unable to write code for the below steps but I do have an mental model what to do, Once I saw the ref code from ChatGPT (I only look into that code for 5 seconds) I wrote total code for all three Jobs myself
// Sleep Job
pub struct SleepJob {
    seconds: u64,
}
// Sleep struct with seconds prop that takes a number (I tried to change it to u8, but somhow it only accepts the u64)

impl Job for SleepJob {
    fn name(&self) -> &str {
        "Sleep Job"
    }

    fn is_cpu_bound(&self) -> bool {
        false // Sleep job is an async so I am returing false (I am unable to identify weather this job is an I/O bound or CPU bound, So I return what was present in the assignment)
    }
}
// I am implementing the Job trait to the SleepJob struct, I need to write the defenation of the functions cause they don't have a defualt defination

#[async_trait] // Who are you?
impl AsyncJob for SleepJob {
    async fn run_async(&self) -> String {
        trpl::sleep(Duration::from_secs(self.seconds)).await;
        format!("Slept for {}", self.seconds)
    }
}
// I am using trpl to make the code to stop here for the N seconds I pass, using await cause it will return an future and a future is lazy so it won't execute until I await it!

// Sum Job
pub struct SumJob {
    num: u32,
}
// BLAH BLAH BLAH I think I don't have to explaing the below two jobs, they are nothing but some programming logic

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
            sum += i + 1 // Doing i + 1 cause it will start from 0 and I need i value from 1, I think I also do this `1..sum.num + 1` am I right?
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

#[async_trait] // Are you zoro? you look lost.
impl AsyncJob for FetchJob {
    async fn run_async(&self) -> String {
        // The below code is from ch 17.1 what it does is it give the title from the URL provided (Clasic WEB SCRAPING, I did selinum this is a piece of cake for me)
        let response = trpl::get(&self.url).await.text().await; //Blah Blah Blah get will return an Response I am using text() to make it to convert to an String and this text is also returing an impl to Future so I had to await it
        Html::parse(&response)
            .select_first("title")
            .map(|title| title.inner_html())
            .expect("Error while fetching the URL's title")
    }
}

// IMPORTANT - I had to have just a sec glace of to the ref code from the ChatGPT to get to what fields to put here, I stil need to improve at my rust logic building
pub struct JobRunner {
    async_handlers: Vec<JoinHandle<()>>,
    thread_handlers: Vec<thread::JoinHandle<()>>,
}
// Even through I had to look code for the ref, I later copied it from ChatGPT I was unable to guess the types for it!

// The below code is nothing but writing by seeing the code from chatGPT, But I know what I am writing
impl JobRunner {
    // Creating new fun to have an instance of the hadler vectors
    fn new() -> Self {
        Self {
            async_handlers: Vec::new(),
            thread_handlers: Vec::new(),
        }
    }

    // This will execute the async jobs
    fn submit_async_jobs<J>(&mut self, job: Arc<J>)
    where
        J: Job + AsyncJob + 'static, // I gave here, Like what is this? I was ok upto AsyncJob cause at the end I am executing the Job which is releted to AsyncJob, I don't ntg about these `+ Send + Sync + 'static` trait bonds through
    {
        let name = job.name().to_string(); // Taking the ownership of the name here

        let handle = tokio::spawn(async move {
            let result = job.run_async().await;
            println!("[Async] {} finished: {}", name, result);
        });
        self.async_handlers.push(handle); // pushing this handler to the async handler vector
    }

    // Same explanation BTW
    fn submit_thread_jobs<J>(&mut self, job: Arc<J>)
    where
        J: Job + BlockingJob + 'static, // `+ Send + Sync + 'static` Who are you guys, Btw do you know this `#[async_trait]` guy, you guys are frineds with him, Losty Guys?
    {
        let name = job.name().to_string();
        let handle = thread::spawn(move || {
            let result = job.run_blocking();
            println!("[Blocking] {} finished: {}", name, result);
        });
        self.thread_handlers.push(handle); // pushing this handler to the thread handler vector
    }
}

#[tokio::main] // We need the parent funtion of the async functions to act asa state manager, since we are making the main funtion itself an async, we are using tokio main to have a state here! (I might wrong here, but this is an high level explanation)
// writing main async to make it an async funtion, So I can have access to await in the main
async fn main() {
    // The following code is a COPY, But I do know what's going on
    let mut runner = JobRunner::new();

    // I know Arc make you to have multiple owners, But why are we using it here?
    let sleep_job = Arc::new(SleepJob { seconds: 2 });
    let sum_job = Arc::new(SumJob { num: 10 });
    let fetch_job = Arc::new(FetchJob {
        url: "https://axna.vercel.app/".to_string(),
    });

    // Running the jobs with corsponding function calls
    runner.submit_async_jobs(sleep_job);
    runner.submit_async_jobs(fetch_job);
    runner.submit_thread_jobs(sum_job);

    // Joining all the async functions in the async handler to await them to wait until all the jobs executed
    for i in runner.async_handlers {
        i.await.unwrap();
    }

    // Joining all the thread functions in the thread handler to await them to wait until all the jobs executed
    for j in runner.thread_handlers {
        j.join().unwrap()
    }

    println!("Job(s) execuation is complected"); // Printing I am Done (Finally!)
}
