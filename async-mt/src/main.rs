fn test() {
    println!("Hello, test!");
}


fn main() {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();

    pool.join(test, test);
}
