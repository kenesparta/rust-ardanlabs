# Read/Write locks

```rust
use once_cell::sync::Lazy;
use std::sync::RwLock;

static USERS: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(build_users()));

fn build_users() -> Vec<String> {
    vec!["Alice".to_string(), "Bob".to_string()]
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| loop {
        println!("Current Users in a thread");
        let users = USERS.read().unwrap();
        println!("{users:?}");
        std::thread::sleep(std::time::Duration::from_secs(3))
    });

    loop {
        println!("Enter a name to add to the user list");
        let input = read_line();
        if input == "q" {
            break;
        }
        let mut lock_users = USERS.write().unwrap();
        lock_users.push(input);
    }
}

```

# Deadlocks, panics and Poisoning
```rust
use std::sync::Mutex;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn main() {
    let lock = MY_SHARED.lock().unwrap();

    if let Ok(_lock) = MY_SHARED.try_lock() {
        print!("Got the lock");
    } else {
        print!("No Lock");
    }
}
```

```rust
use std::sync::Mutex;
use std::thread;

static MY_SHARED: Mutex<u32> = Mutex::new(3);

fn poisoner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("The poisoner strikes!")
}

fn main() {
    let handle = thread::spawn(poisoner);
    println!("trying to return from the thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");
}
```

# Sharing data with Lock-Free Structs
```rust
use dashmap::DashMap;
use once_cell::sync::Lazy;

static SHARED: Lazy<DashMap<u32, u32>> = Lazy::new(DashMap::new);

fn main() {
    for n in 0..100 {
        std::thread::spawn(move || {
            if let Some(mut v) = SHARED.get_mut(&n) {
                *v += 1
            } else {
                SHARED.insert(n, n);
            }
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("{:#?}", SHARED);
}
```

# Parking threads
```rust
fn parkable_thread(n: u32){
    loop {
        std::thread::park();
        println!("Thread {n} is unparked briefly");
    }
}

fn read_line() -> String{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.trim().to_string()
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let thread  = std::thread::spawn(move || {
            parkable_thread(i);
        });
        threads.push(thread);
    }

    loop {
        println!("Thread to unpark: ");
        let input = read_line();
        if input == "q" {
            break;
        }
        if let Ok(number) = input.parse::<usize>(){
            if number < 10 {
                threads[number].thread().unpark()
            }
        }
    }
}
```

# Sending data between threads
```rust

```