use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello from Tokio!");
    sleep(Duration::from_millis(200)).await;
}
