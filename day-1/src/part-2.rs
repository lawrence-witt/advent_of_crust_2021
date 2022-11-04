use common;

fn str_to_i32(value: &str) -> i32 {
    return value.parse::<i32>().unwrap();
}

fn str_arr_to_i32_vec(value: &[String]) -> Vec<i32> {
    return value.iter().map(|v| str_to_i32(v)).collect();
}

fn sum_i32_vec(value: &Vec<i32>) -> i32 {
    return value.iter().sum();
}

fn main() {
    let input = common::read_new_line_input(1);
    let mut count = 0;
    for n in 0..input.len() {
        if n < 3 {
            continue;
        };
        let prev = sum_i32_vec(&str_arr_to_i32_vec(&input[n - 3..=n - 1]));
        let curr = sum_i32_vec(&str_arr_to_i32_vec(&input[n - 2..=n]));
        if curr > prev {
            count += 1;
        }
    }
    println!("{}", count);
}
