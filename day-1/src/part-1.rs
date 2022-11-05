use common::*;

fn main() {
    let input = read_new_line_input(1);
    let mut count = 0;
    for n in 0..input.len() {
        if n == 0 {
            continue;
        }
        if str_to_i32(&input[n]) > str_to_i32(&input[n - 1]) {
            count += 1;
        }
    }
    println!("{}", count);
}
