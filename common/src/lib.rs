use std::fs;

pub fn read_rn_input(day: u8) -> Vec<String> {
    return fs::read_to_string(format!("day-{day}/input.txt"))
        .expect("failed to read file path")
        .trim()
        .split("\r\n")
        .map(String::from)
        .collect();
}
