## Hello Asyc
```rust
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;

fn do_something_sync() {
    println!("not async!");
}

async fn say_hello() {
    println!("hello");
    join!(say_world(), say_goodbye());
    let n = double(4).await;
    println!("n: {}", n);

    let futures_arr = vec![double(1), double(2), double(3), double(4)];
    let res = join_all(futures_arr).await;
    println!("res: {:?}", res);

    do_something_sync();
}

async fn say_world() {
    println!("world");
}

async fn say_goodbye() {
    println!("goodbye");
}

async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {
    block_on(say_hello());
}
```

## Async using Tokio

```rust
async fn hello() {
    println!("hello tokio");
}

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(4)
        .build()
        .unwrap();
    
    rt.block_on(hello());
}
```

Easiest way
```rust
async fn hello() {
    println!("hello tokio");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;
}
```

```rust
async fn hello() -> u32 {
    println!("hello tokio");
    3
}

async fn hello2() -> u32 {
    println!("hello tokio2");
    4
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;

    let result = tokio::join!(hello(), hello2());
    println!("{:?}", result);
}
```

```rust
async fn hello() -> u32 {
    println!("hello tokio");
    3
}

async fn ticker() {
    for n in 0..50 {
        println!("tick {}", n);
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(ticker());
    hello().await;
}
```

Complicate it a little more
```rust
async fn hello() -> u32 {
    println!("hello tokio");
    3
}

async fn ticker() {
    for n in 0..10 {
        println!("tick {}", n);
        // Go ahead to other tasks
        tokio::task::yield_now().await;
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let _ = tokio::join!(
        tokio::spawn(hello()),
        tokio::spawn(ticker()),
        tokio::spawn(ticker())
    );
    println!("done");
}
```

## Blocking tasks
```rust
async fn hello_delay(task: u64, time: u64) {
    println!("Task {task} has started");
    // std::thread::sleep(std::time::Duration::from_millis(time));
    tokio::time::sleep(std::time::Duration::from_millis(time)).await;
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
```

```rust
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
```

## Testing tokio
```rust
async fn double(n: u32) -> u32 {
    n * 2
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn will_not_compile() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        assert_eq!(rt.block_on(double(2)), 4)
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_easy_way() {
        assert_eq!(double(2).await, 4)
    }
}
```

## Handling errors

```rust
use serde::Deserialize;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
enum UserError {
    #[error("No user found")]
    NoUserFound,

    #[error("Too many users found")]
    TooManyUsers,
}

fn maybe_read_file() -> Result<String, std::io::Error> {
    let my_file = Path::new("myfile.txt");
    std::fs::read_to_string(my_file)
}

fn file_to_uppercase() -> Result<String, std::io::Error> {
    let contents = maybe_read_file()?;
    Ok(contents.to_ascii_uppercase())
}

#[derive(Deserialize)]
struct User {
    user: String,
}

// Instead of this type GenericResul we can use the anyhow crate (anyhow::Result)
// type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

fn load_user() -> Result<Vec<User>, UserError> {
    let my_path = Path::new("myfile.txt");
    let raw_text = std::fs::read_to_string(my_path).map_err(|_| UserError::NoUserFound)?;
    let users: Vec<User> = serde_json::from_str(&raw_text).map_err(|_| UserError::NoUserFound)?;
    // anyhow::bail!("We can't go on!");
    Ok(users)
}

fn main() {
    if let Ok(content) = file_to_uppercase() {
        println!("Contents: {}", content);
    }

    let my_file = Path::new("myfile.txt");
    let content = std::fs::read_to_string(my_file);
    match content {
        Ok(content) => println!("{}", content),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found - myfile.txt"),
            _ => println!("ERROR: {:#?}", e),
        },
    }
}
```
