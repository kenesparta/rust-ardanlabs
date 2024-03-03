fn main() {
    let mut n: i32 = 5;
    n += 1;
    // Scope of variables
    {
        let n: i32 = 5;
        println!("n_ = {n}");
    }
    println!("n = {n}");

    // let w: i32 = 8;
    // let w: () = {
    //     let n: i32 = 6;
    // };
    // println!("w = {w}");

    // Functional

    let name = "Ken".to_string();
    let name = greet(name);
    greet(name);

    let namebr = "Ken".to_string();
    greet_borrow(&namebr);

    let input = read_line();
    println!("You typed: [{input}]")
}

fn double(n: i32) -> i32 {
    n * 2
}

fn greet(s: String) -> String {
    println!("Hello {s}");
    s
}

fn greet_borrow(s: &String) {
    println!("{s}")
}

fn great_borrow_mut(s: &mut String) {
    *s = format!("Hello {s}")
}

fn double_or_nothing(n: i32) -> i32 {
    if n > 0 {
        n * 2;
    }
    0
}
