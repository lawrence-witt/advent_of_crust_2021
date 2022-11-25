extern crate common;
mod utils;

fn main() {
    let input = common::read_input(9, "\n");
    let mut result: u32 = 0;
    for i in 0..input.len() {
        let line = &input[i];
        for j in 0..line.len() {
            let value = line.get(j..j+1).unwrap().parse::<u8>().unwrap();
            if utils::value_is_lowpoint(value, i, j, &input) {
                result += 1 + u32::from(value);
            }
        };
    }
    println!("{:#?}", result);
}