# 🧵🧠 Rust Concurrency Practice Task (Medium–Hard)

✅ Threads  
✅ Message Passing  
✅ Shared State  
✅ `Arc<Mutex<T>>`  
❌ No unsafe  
❌ No cheating  

---

## 📌 Task Title
**Multi-Threaded Log Analyzer with Shared State + Channels**

---

## 🎯 Objective
Build a Rust program that analyzes a log file using **multiple threads**, combining:

- **Message Passing** (`mpsc`)
- **Shared Mutable State** (`Arc<Mutex<_>>`)

This mirrors **real-world Rust concurrency**, not textbook demos.

---

## 🗂 Problem Statement

You are given a text file called `log.txt`.

Each line starts with a log level:

```
INFO Server started
ERROR Database connection failed
WARN Cache miss
INFO User logged in
ERROR Timeout
```

---

## 🧠 Your Job

1. Read the log file line by line
2. Distribute lines to multiple worker threads using **channels**
3. Each worker:
   - Parses the log level
   - Updates a **shared global counter**
4. Main thread:
   - Waits for all workers
   - Prints the final aggregated result

---

## 🧱 Requirements (Strict)

### Concurrency Rules
- Use **at least 4 worker threads**
- Use:
  - `std::sync::mpsc`
  - `Arc<Mutex<HashMap<String, usize>>>`
- ❌ No global variables
- ❌ No `unsafe`
- ❌ No external crates

---

## 🔐 Shared State Rules

- The shared state must:
  - Be created in `main`
  - Be wrapped in `Arc<Mutex<_>>`
  - Be cloned into each worker thread
- Workers must:
  - Lock the mutex
  - Update counts
  - Release the lock ASAP

---

## 📥 Input
File: `log.txt`

Assumptions:
- File fits in memory
- Each line starts with `INFO`, `WARN`, or `ERROR`

---

## 📤 Output (Example)

```
Final Log Summary:
INFO  -> 120
WARN  -> 45
ERROR -> 32
```

Order does NOT matter.

---

## 🏗 Suggested Architecture (Recommended)

```
main thread
 ├── reads file
 ├── sends lines → work_tx
 ├── holds Arc<Mutex<HashMap>>
 ├── joins all workers
 └── prints final result

worker threads (N)
 ├── receive lines
 ├── lock shared map
 ├── update count
 └── unlock
```

---

## ⚠️ Things Your Senior WILL Check

Make sure you can explain:

1. Why `Arc` is needed
2. Why `Mutex` is needed
3. Why `Receiver` is not shared
4. Why `drop(tx)` is mandatory
5. How deadlocks are avoided
6. Why lock scope should be minimal

---

## ⭐ Bonus Challenges (Flex Mode 💪)

Try one or more:

1. Add a second shared state:
   - Total number of lines processed
2. Add artificial delay in workers and observe contention
3. Use `try_lock()` and handle failure
4. Measure execution time with different worker counts
5. Replace `HashMap<String, usize>` with an enum-based design

---

## 🧪 What This Task Proves

✔ You understand **ownership across threads**  
✔ You know when to use **channels vs mutexes**  
✔ You can reason about **deadlocks & contention**  
✔ You’re no longer “Rust beginner”  

---

## 🦀 Final Boss Question (Ask Yourself)

> Why is this design safe even though multiple threads mutate shared state?
