use std::{
    collections::HashMap,
    fs, process,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread,
};

fn main() {
    let log_contents = fs::read_to_string("log.txt").unwrap_or_else(|err| {
        println!("Unable to fetch file contents {}", err);
        process::exit(1)
    });

    let shared_state = Arc::new(Mutex::new(HashMap::<String, usize>::new()));

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let worker_count = 4;
    let mut handles = Vec::new();

    for i in 0..worker_count {
        let rx = Arc::clone(&rx);
        let counts = Arc::clone(&shared_state);

        let thread = thread::spawn(move || {
            loop {
                let line = {
                    let rx = rx.lock().unwrap();
                    rx.recv()
                };

                match line {
                    Ok(li) => {
                        if let Some(li) = li.split_whitespace().next() {
                            let mut count = counts.lock().unwrap();
                            *count.entry(li.to_string()).or_insert(0) += 1;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        handles.push(thread);
    }

    for line in log_contents.lines() {
        tx.send(line.to_string()).unwrap()
    }

    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Log Summery");

    let result = shared_state.lock().unwrap();
    for (le, count) in result.iter() {
        println!("{} -> {}", le, count);
    }
}
