# 🦀 Rust Intermediate Task – Modular CLI Task Manager

This project is designed for **beginners moving toward intermediate Rust**.
You will build a **command-line Task Manager** while learning **modules, crates, error handling, and testing**.

---

## 🎯 Goals

By completing this task, you will learn:

- Rust module system (`mod`, `pub`, file-based modules)
- Crate structure (`main.rs` + `lib.rs`)
- Structs and enums
- Custom error types
- `Result<T, E>` based error handling
- Basic unit testing
- Clean project organization (real-world style)

---

## 📁 Project Structure (Required)

Your project **must follow this structure**:

```
task_manager/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    ├── task/
    │   ├── mod.rs
    │   └── model.rs
    ├── storage/
    │   ├── mod.rs
    │   └── memory.rs
    └── error.rs
```

---

## 🧩 Functional Requirements

### 1️⃣ Task Model (`task::model`)

Create a `Task` struct:

```rust
pub struct Task {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
```

**Required methods**
- `new(id: u32, title: String) -> Task`
- `mark_done(&mut self)`

---

### 2️⃣ Task Storage (`storage::memory`)

Create an in-memory task store:

```rust
pub struct TaskStore {
    tasks: Vec<Task>,
}
```

**Required methods**
- `add_task(title: String) -> Task`
- `list_tasks() -> Vec<&Task>`
- `complete_task(id: u32) -> Result<(), TaskError>`

---

### 3️⃣ Custom Error Handling (`error`)

Create a custom error enum:

```rust
pub enum TaskError {
    TaskNotFound(u32),
}
```

**Requirements**
- Implement `std::fmt::Display`
- Implement `std::error::Error`

---

### 4️⃣ Library Interface (`lib.rs`)

Expose only the public API:

```rust
pub mod task;
pub mod storage;
pub mod error;
```

❗ Do **not** expose internal fields unnecessarily.

---

### 5️⃣ Command Line Interface (`main.rs`)

Support the following commands:

```bash
cargo run add "Learn Rust modules"
cargo run list
cargo run done 1
```

**CLI rules**
- Use `std::env::args`
- Use `match` for command parsing
- Handle errors gracefully
- ❌ No `panic!`
- ❌ No `unwrap()` in production code

---

### 6️⃣ Unit Tests (Mandatory)

Write tests for:

- Adding a task
- Completing a task
- Completing a non-existing task (must return an error)

---

## 🚫 Constraints

- ❌ No `unwrap()` or `expect()` in production code
- ❌ No global variables
- ❌ No `panic!` for normal flow
- ✅ Use `Result<T, E>`
- ✅ Use modules properly
- ✅ Code must pass `cargo check`

---

## ⭐ Bonus (Optional)

- Replace `bool` with `TaskStatus` enum
- Add `remove_task(id)`
- Add `#[derive(Debug, Clone)]`
- Persist tasks using a file
- Convert this into a workspace with multiple crates

---

## 🏁 Expected Outcome

After completing this project, you should be comfortable with:

- Structuring real Rust projects
- Writing modular, maintainable code
- Handling errors the Rust way
- Writing and running unit tests

---

Happy hacking 🦀🔥