extern crate common;

fn main() {
    let input = common::read_new_line_input(1);
    let mut count = 0;
    for n in 0..input.len() {
        if n == 0 {
            continue;
        }
        if common::str_to_i32(&input[n]) > common::str_to_i32(&input[n - 1]) {
            count += 1;
        }
    }
    println!("{}", count);
}
