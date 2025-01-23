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

## Sharing Data with atomics

A counter using unsafe, without atomics

```rust
// Creating a global variable
static mut COUNTER: i32 = 0;

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = thread::spawn(|| {
            for _ in 0..1100 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        handles.push(handle)
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    unsafe { println!("{COUNTER}") }
}

```

With `std::sync::atomic`:

```rust
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

// Creating a global variable
static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = thread::spawn(|| {
            for _ in 0..1100 {
                COUNTER.fetch_add(1, Relaxed);
            }
        });
        handles.push(handle)
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("{}", COUNTER.load(Relaxed));
}
```

## Sharing data with Mutexes

Is slower than Atomics.

```rust
use std::sync::Mutex;
use std::thread;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

fn main() {
    let mut handles = Vec::new();
    for _ in 0..10 {
        let handle = thread::spawn(|| {
            let mut lock = NUMBERS.lock().unwrap();
            lock.push(1);
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    let lock = NUMBERS.lock().unwrap();
    println!("{:#?}", lock)
}
```

# Documentation:

- The top of our library (each public module) should have a documentation comment `//!` indicates the scope level
  documentation. This is where we should describe the purpose of the library.
- Function level Doc: you should also document every public function and struct, we can do that using this `///`

