fn hello_thread() {
    print!("Hello from thread\n");
}

fn main() {
    print!("Hello from MAIN thread\n");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().expect("");
}
