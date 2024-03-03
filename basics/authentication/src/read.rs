pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("TODO: panic message");
    input.trim().to_string()
}
