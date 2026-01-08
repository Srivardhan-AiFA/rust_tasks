# Task 1: Generic Notification Processing Engine (Rust)

## 🎯 Goal
Build a **type-safe, extensible notification processing engine** using **Generics and Traits** (no lifetimes).

This task mirrors **real-world Rust design patterns** used in backend systems, SDKs, and libraries.

---

## 🧠 Concepts You Will Practice
- Generic structs and functions
- Traits as behavior contracts
- Trait bounds (`T: Trait`)
- Static dispatch
- Open–Closed Principle (extend without modifying core logic)
- Strategy Pattern (via traits)

---

## 📦 Problem Statement

You are building a **Notification Engine** that can send messages using different channels
(Email, SMS, Push, etc.).

Each channel:
- Has its own sending logic
- May validate messages differently
- Can be added later **without changing existing code**

---

## 🏗️ Requirements

### 1️⃣ Define a `Notifier` trait
The trait represents a notification strategy.

```rust
pub trait Notifier {
    fn send(&self, recipient: &str, message: &str) -> Result<(), String>;
}
```

---

### 2️⃣ Create concrete implementations
Implement `Notifier` for at least **three** types:

- `EmailNotifier`
- `SmsNotifier`
- `PushNotifier`

Each implementation should:
- Simulate sending (use `println!`)
- Fail validation if input is invalid (empty message, invalid address, etc.)

---

### 3️⃣ Create a generic `NotificationService<T>`
This struct should:
- Be generic over `T`
- Accept any notifier that implements `Notifier`

```rust
pub struct NotificationService<T: Notifier> {
    notifier: T,
}
```

---

### 4️⃣ Implement service logic
Add methods like:
- `new(notifier: T) -> Self`
- `notify(&self, recipient: &str, message: &str)`

The service **must not know** what kind of notifier it is using.

---

### 5️⃣ Add a generic retry mechanism
Create a **generic function**:

```rust
fn retry_send<T: Notifier>(
    notifier: &T,
    recipient: &str,
    message: &str,
    retries: u8,
) -> Result<(), String>
```

---

### 6️⃣ Demonstrate usage in `main.rs`
- Create different notifier types
- Wrap them in `NotificationService`
- Call notify and retry logic

---

## 🧩 Design Pattern Used

### Strategy Pattern (Rust-style)
- `Notifier` = strategy interface
- Concrete notifiers = strategies
- `NotificationService` = context

This pattern is widely used in:
- Logging frameworks
- Payment gateways
- Transport layers
- Cloud SDKs

---

## 📂 Suggested Project Structure

```
notification_engine/
├── src/
│   ├── main.rs
│   ├── notifier.rs
│   ├── service.rs
│   └── providers/
│       ├── email.rs
│       ├── sms.rs
│       └── push.rs
└── Cargo.toml
```

---

## ✅ Completion Checklist

- [ ] Trait implemented correctly
- [ ] Generic struct compiles
- [ ] No `Box<dyn Trait>` used (static dispatch only)
- [ ] No lifetimes used
- [ ] New notifier can be added without touching existing code
- [ ] Meaningful error handling with `Result`

---

## 🚀 Stretch Goals (Optional)
- Add logging trait
- Add a mock notifier for testing
- Add unit tests using generics

---

## 💡 Learning Outcome

After finishing this task, you should confidently understand:
- Why Rust uses generics
- How traits enable polymorphism
- How real Rust libraries are designed

---

Happy Rusting 🦀  
This task is **intentionally realistic** — struggle is expected and good.
