use std::fs;

pub fn read_input(day: u8, delimiter: &str) -> Vec<String> {
    return fs::read_to_string(format!("day-{day}/input.txt"))
    .expect("should read file path")
    .trim()
    .split(delimiter)
    .map(|e| String::from(String::from(e).trim()))
    .collect();
}

pub fn read_new_line_input(day: u8) -> Vec<String> {
    return fs::read_to_string(format!("day-{day}/input.txt"))
        .expect("should read file path")
        .trim()
        .split("\n")
        .map(|e| String::from(String::from(e).trim()))
        .collect();
}

pub fn str_to_i32(value: &str) -> i32 {
    return value.parse::<i32>().unwrap();
}
