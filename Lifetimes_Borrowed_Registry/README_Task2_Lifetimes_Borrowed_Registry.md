# Task 2: Borrowed Data Registry (Mastering Lifetimes in Rust)

## 🎯 Goal
Build a **Borrowed Data Registry** that stores and returns **references instead of owned data**.

This task is designed to make you **struggle productively** with:
- Lifetimes on structs
- Lifetimes in method signatures
- Lifetime relationships between inputs and outputs
- Why Rust *must* reject certain designs

⚠️ This task is intentionally harder than Task 1.

---

## 🧠 Concepts You Will Practice
- Explicit lifetime parameters (`'a`)
- Structs holding references
- Lifetime elision rules (and where they fail)
- Returning references safely
- Tying output lifetimes to inputs
- Designing APIs that *respect ownership*

---

## 🏗️ Problem Statement

You are building a **Registry** that stores **borrowed records** (not owned).

This registry:
- Does NOT own the data
- Only stores references
- Must guarantee all references remain valid
- Should expose safe query APIs

Think of this like:
- A cache over external data
- A view layer over database rows
- An index over in-memory records

---

## 📦 Data Model

### Record
```rust
pub struct Record {
    pub id: u32,
    pub name: String,
}
```

You will **NOT** store `Record` values inside the registry.  
You will only store `&Record`.

---

## 🧩 Core Task Requirements

### 1️⃣ Create `RecordRegistry`
Create a struct that stores references to records.

```rust
pub struct RecordRegistry<'a> {
    records: Vec<&'a Record>,
}
```

👉 You **must** explain to yourself what `'a` means here.

---

### 2️⃣ Implement constructor
```rust
impl<'a> RecordRegistry<'a> {
    pub fn new() -> Self;
}
```

No lifetimes in arguments yet — but the struct still needs one.

---

### 3️⃣ Add records by reference
```rust
pub fn add(&mut self, record: &'a Record);
```

⚠️ This is where many people get stuck.

Ask yourself:
- Why must the input reference use `'a`?
- Why not a different lifetime?

---

### 4️⃣ Query by ID (HARD PART)
Implement:
```rust
pub fn get_by_id(&self, id: u32) -> Option<&'a Record>;
```

This method must:
- Return a reference
- Tie the returned reference to the registry lifetime
- Compile without cloning or allocating

This is the **core lifetime challenge**.

---

### 5️⃣ Longest name (lifetime reasoning)
Implement:
```rust
pub fn longest_name(&self) -> Option<&'a str>;
```

Rules:
- You may NOT allocate new strings
- You may NOT clone
- You must return a borrowed `&str`

This forces you to reason about **nested borrows**:
`&Record -> &String -> &str`

---

## 🚨 Intentional Traps (You SHOULD hit these)

You will likely encounter:
- “borrowed value does not live long enough”
- “cannot return reference to local variable”
- lifetime mismatch errors
- confusion between `'a` and method-local lifetimes

These errors are **the point** of the task.

---

## 📂 Suggested Project Structure

```
borrowed_registry/
├── src/
│   ├── main.rs
│   ├── record.rs
│   └── registry.rs
└── Cargo.toml
```

---

## 🧪 Example Usage (this MUST compile)

```rust
fn main() {
    let r1 = Record { id: 1, name: "Alice".to_string() };
    let r2 = Record { id: 2, name: "Bob".to_string() };

    let mut registry = RecordRegistry::new();
    registry.add(&r1);
    registry.add(&r2);

    let record = registry.get_by_id(1).unwrap();
    println!("{}", record.name);

    let longest = registry.longest_name().unwrap();
    println!("{}", longest);
}
```

⚠️ **Do not change this usage**.  
Fix the lifetimes to make it work.

---

## ✅ Completion Checklist

- [ ] No `String` clones
- [ ] No `to_string()`
- [ ] No owned `Record` inside registry
- [ ] All data is borrowed
- [ ] Explicit lifetimes used
- [ ] Compiler errors understood, not ignored

---

## 🚀 Stretch Goals (Very Hard)

- Add `remove(&mut self, id: u32) -> Option<&'a Record>`
- Add `iter(&self) -> impl Iterator<Item = &'a Record>`
- Explain (in comments) why `'static` is WRONG here

---

## 🧠 Learning Outcome

After this task, you should:
- Stop fearing lifetime annotations
- Understand *why* Rust needs them
- Design APIs around borrowing, not ownership
- Read lifetime errors without panic

If Task 1 made you confident,  
**Task 2 will make you dangerous** 🦀🔥

---

Good luck. Struggle is expected.
