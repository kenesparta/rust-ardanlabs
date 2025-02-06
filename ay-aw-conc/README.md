# Hello Asyc
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

# Async using Tokio

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