use std::io;

fn main() {
    println!("What is your name?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("Hello, {}!", input.trim());

}
