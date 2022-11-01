use std::fs;

fn read_input() -> Vec<String> {
    return fs::read_to_string("day-1/input.txt")
        .expect("failed to read file path")
        .trim()
        .split("\r\n")
        .map(String::from)
        .collect();
}

fn main() {
    let input = read_input();
    println!("{:?}", input);
}
