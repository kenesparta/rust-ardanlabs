## Spawn threads with parameters
```rust
fn hello_thread(n: i32) {
    print!("Hello from thread {n}\n");
}

fn do_math(i: u32) -> u32 {
    let mut n = i + 1;
    for _ in 0..10 {
        n *= 2;
    }
    n
}

fn main() {
    print!("Hello from MAIN thread\n");

    let mut thread_handles_vector = Vec::new();
    for i in 0..10 {
        let thread_handler = std::thread::spawn(move || do_math(i));
        thread_handles_vector.push(thread_handler)
    }

    thread_handles_vector
        .into_iter()
        .for_each(|h| println!("{}", h.join().expect("")));
}
```
## Dividing workloads
```rust
fn main() {
    const N_THREADS: usize = 5;
    let to_add: Vec<u32> = (0..5_000).collect();
    let mut thread_handles_workers = Vec::new();
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();
        thread_handles_workers.push(std::thread::spawn(move || my_chunk.iter().sum::<u32>()))
    }

    let mut total_sum = 0;
    for handle in thread_handles_workers {
        total_sum += handle.join().expect("");
    }

    println!("sum is {total_sum}")
}
```

## ThreadBuilder
```rust
use std::thread;

fn my_thread() {
    println!(
        "Hello from thread named {}",
        thread::current().name().unwrap()
    )
}

fn main() {
    thread::Builder::new()
        .name("Named Thread".to_string())
        .stack_size(std::mem::size_of::<usize>())
        .spawn(my_thread)
        .unwrap();
    println!()
}
```

## Scoped Threads
```rust
use std::thread;

fn main() {
    const N_THREADS: usize = 5;
    let to_add: Vec<u32> = (0..5_000).collect();
    let chunks = to_add.chunks(N_THREADS);
    let total_sum = thread::scope(|s| {
        let mut thead_handles_vector = Vec::new();

        for chunk in chunks {
            let thread_handle = s.spawn(move || chunk.iter().sum::<u32>());
            thead_handles_vector.push(thread_handle);
        }

        thead_handles_vector
            .into_iter()
            .map(|h| h.join().unwrap())
            .sum::<u32>()
    });
    println!("Sum: {total_sum}")
}
```