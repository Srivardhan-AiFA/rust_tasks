# Assignment: Concurrent Task Execution Framework in Rust

## Background

Modern backend systems must handle a mix of:

* **CPU-bound tasks** (e.g., compression, hashing)
* **IO-bound tasks** (e.g., network requests, file reads)

Rust provides:

* **Traits and trait bounds** for abstraction
* **Async/await and Futures** for IO concurrency
* **Threads and synchronization primitives** for CPU parallelism

In this assignment, you will design and implement a **hybrid task execution framework** that:

* Accepts different kinds of tasks through a common interface
* Executes tasks concurrently using the appropriate execution model
* Ensures thread safety, correctness, and clean abstraction boundaries

---

## Objective

Build a **task execution framework** that:

1. Allows multiple kinds of tasks to be registered dynamically
2. Uses **async execution** for IO-bound tasks
3. Uses **thread pools** for CPU-bound tasks
4. Uses **traits and trait bounds** to enforce safety and flexibility
5. Collects and reports task execution results

---

## Functional Requirements

### 1. Task Abstraction

Define a trait representing a generic task:

```rust
pub trait Task: Send + Sync {
    fn name(&self) -> &'static str;
    fn is_cpu_bound(&self) -> bool;
}
```

#### Additional Requirements

* Tasks **must be shareable across threads**
* Task behavior must be encapsulated behind traits
* Tasks may carry internal state

---

### 2. Async Task Interface

Define an async execution trait:

```rust
#[async_trait::async_trait]
pub trait AsyncExecutable {
    async fn execute_async(&self) -> TaskResult;
}
```

#### Requirements

* Only IO-bound tasks should implement this trait
* Async execution must be non-blocking
* Use `.await` correctly

---

### 3. Blocking Task Interface

Define a blocking execution trait:

```rust
pub trait BlockingExecutable {
    fn execute_blocking(&self) -> TaskResult;
}
```

#### Requirements

* Only CPU-bound tasks should implement this trait
* Must be safe to run in a thread pool
* Must not rely on async runtimes

---

### 4. Task Implementations

Implement **at least three tasks**:

| Task            | Type      | Description                          |
| --------------- | --------- | ------------------------------------ |
| `HttpFetchTask` | IO-bound  | Fetch data from a URL asynchronously |
| `SleepTask`     | IO-bound  | Async delay and return duration      |
| `HashTask`      | CPU-bound | Compute a SHA-256 hash               |

#### Rules

* Each task must implement `Task`
* Each task must implement **only one execution trait**
* Use appropriate libraries (`tokio`, `sha2`, etc.)

---

### 5. Task Scheduler

Create a `TaskScheduler` responsible for dispatching tasks.

```rust
pub struct TaskScheduler {
    // internal fields
}
```

#### Responsibilities

1. Accept tasks via:

```rust
fn submit(&self, task: Arc<dyn Task>);
```

2. Route tasks to:

* Async executor for IO-bound tasks
* Thread pool for CPU-bound tasks

3. Execute multiple tasks concurrently

4. Prevent blocking the async runtime with CPU-bound work

---

### 6. Thread Pool Implementation

You must implement a **basic thread pool** using:

* `std::thread`
* `mpsc::channel`
* `Arc<Mutex<_>>`

#### Requirements

* Configurable number of worker threads
* Graceful shutdown
* Safe task sharing

---

### 7. Result Collection

Define:

```rust
pub enum TaskResult {
    Success(String),
    Failure(String),
}
```

#### Requirements

* Collect results from all tasks
* Preserve task names in output
* Handle failures gracefully

---

### 8. Concurrency & Safety Constraints

Your solution **must**:

* Avoid data races
* Avoid deadlocks
* Use `Send` and `Sync` correctly
* Justify all trait bounds used

---

## Non-Functional Requirements

### Code Quality

* Clear module separation
* Idiomatic Rust
* Proper error handling (no `unwrap()` in core logic)

### Testing

* Unit tests for:

  * Thread pool
  * Task execution
* Integration test for scheduler

---

## Bonus Challenges (Optional)

1. **Task Timeout**

   * Cancel tasks exceeding a time limit

2. **Retry Mechanism**

   * Retry failed async tasks with backoff

3. **Task Priorities**

   * Schedule higher-priority tasks first

4. **Metrics**

   * Track execution time per task

---
