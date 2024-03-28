fn hello_thread(n: i32) {
    print!("Hello from thread {n}\n");
}

fn main() {
    print!("Hello from MAIN thread\n");

    let mut thread_handles_vector = Vec::new();
    for i in 0..25 {
        let thread_handler = std::thread::spawn(move || hello_thread(i));
        thread_handles_vector.push(thread_handler)
    }

    thread_handles_vector
        .into_iter()
        .for_each(|h| h.join().expect(""));
}
