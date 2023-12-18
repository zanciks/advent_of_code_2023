use std::fs;

fn main() {
    let mut contents = fs::read_to_string("src/prompt.txt").expect("Error reading prompt.txt");
    for line in contents.lines() {
        println!("{}", line);
    }
    println!("Hello, world!");
}
