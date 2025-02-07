use tokio::task::spawn_blocking;

async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    let _ = spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_millis(time));
    })
    .await;
    println!("Task {task} finished");
}

#[tokio::main]
async fn main() {
    tokio::join!(
        hello_delay(1, 1200),
        hello_delay(2, 100),
        hello_delay(3, 1800),
    );
}
