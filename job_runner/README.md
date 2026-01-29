# Concurrent Job Runner in Rust

## Overview

Many real-world applications need to execute different kinds of work concurrently:

- **IO-bound jobs** (network calls, timers, waiting)
- **CPU-bound jobs** (computations)

Rust provides multiple concurrency tools for this:

- **Traits** to abstract behavior
- **Trait bounds** (`Send`, `Sync`) to ensure thread safety
- **`async/await`** for non-blocking IO
- **OS threads** for parallel CPU work

This project implements a **job runner** that executes different kinds of jobs concurrently using the appropriate mechanism.

---

## Objectives

This program demonstrates how to:

- Define a common job abstraction using traits
- Execute IO-bound jobs asynchronously using **Tokio**
- Execute CPU-bound jobs on **OS threads**
- Use `Send` and `Sync` correctly for thread safety
- Collect and print job results after completion

---

## Design Overview

### Job Abstraction

All jobs implement a shared `Job` trait, which exposes metadata only:

```rust
pub trait Job: Send + Sync {
    fn name(&self) -> &str;
    fn is_cpu_bound(&self) -> bool;
}
```

- Jobs are required to be **thread-safe**
- Execution logic is intentionally *not* part of this trait
- The runner treats jobs as opaque objects

---

### Execution Traits

Execution behavior is separated into **two mutually exclusive traits**.

#### Async Jobs (IO-bound)

```rust
#[async_trait::async_trait]
pub trait AsyncJob {
    async fn run_async(&self) -> String;
}
```

Rules:
- Must be **non-blocking**
- Executed using `tokio::spawn`
- Suitable for timers, network IO, etc.

#### Blocking Jobs (CPU-bound)

```rust
pub trait BlockingJob {
    fn run_blocking(&self) -> String;
}
```

Rules:
- May use CPU freely
- Executed using `std::thread::spawn`
- Must be safe to run on OS threads

> **Important:**  
> Each job implements **exactly one** execution trait.

---

## Job Implementations

This project includes three concrete jobs:

| Job       | Type      | Description |
|----------|-----------|-------------|
| SleepJob | Async     | Sleeps for N seconds and returns a message |
| FetchJob | Async     | Fetches a web page and extracts the title |
| SumJob   | Blocking  | Computes the sum of numbers up to N |

Each job:
- Implements `Job`
- Implements **only one** execution trait
- Encapsulates its own execution logic

---

## Job Runner

The `JobRunner` is responsible for **orchestrating execution**, not doing work itself.

### Responsibilities

- Accept submitted jobs
- Spawn async jobs using `tokio::spawn`
- Spawn blocking jobs using `std::thread::spawn`
- Track execution using join handles
- Wait until **all jobs complete** before exiting

The runner:
- Routes jobs to the correct execution mechanism
- Waits on handles, not time or flags
- Ensures structured concurrency

---

## Result Handling

Each job returns a `String` result.

When a job finishes, the runner prints:

- Job name
- Execution type (`Async` or `Blocking`)
- Result

### Example Output

```
[Async] Sleep Job finished: Slept for 2 seconds
[Blocking] Sum Job finished: The Sum of the 10 is 55
[Async] Fetch Job finished: Example Domain
```

---

## Constraints Followed

- No custom thread pools
- No custom executors
- Async work uses **Tokio**
- Blocking work uses **std::thread**
- No blocking inside async tasks
- All shared data uses `Arc`

---

## Summary

This project demonstrates a clean separation between:

- **Job description**
- **Execution strategy**
- **Concurrency control**

It highlights how Rust’s type system and trait bounds enforce safe and correct concurrent execution at compile time.
