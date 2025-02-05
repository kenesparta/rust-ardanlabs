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