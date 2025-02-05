async fn hello() {
    println!("hello tokio");
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    hello().await;
}
