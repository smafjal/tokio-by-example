# Spawning Tasks

In Tokio, a **task** is an asynchronous unit of execution. Tasks are managed by the Tokio runtime and executed concurrently across threads. They are similar to OS threads, but lightweight and non-blocking.

To execute concurrent operations, use `tokio::spawn`.

---

## The `tokio::spawn` Function

The `tokio::spawn` function takes an `async` block and returns a `JoinHandle`.

```rust,editable
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Spawn a background task
    let handle = tokio::spawn(async {
        println!("Task running concurrently...");
        sleep(Duration::from_millis(500)).await;
        "Task completed!"
    });

    println!("Main task continues working...");

    // Wait for the spawned task to complete
    match handle.await {
        Ok(result) => println!("Task output: {result}"),
        Err(e) => eprintln!("Task panicked: {e}"),
    }
}
```
