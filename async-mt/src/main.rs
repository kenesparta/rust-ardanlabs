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

    let recover_data = lock.unwrap_or_else(|p| {
        println!("Reovering data");
        p.into_inner()
    });
}
