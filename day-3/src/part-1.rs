extern crate common;

fn resolve_bit(n: usize, input: &Vec<String>) -> (String, String) {
    let mut zeros = 0;
    let mut ones = 0;
    for num in input {
        match common::str_to_i32(&String::from(num.as_bytes()[n] as char)) {
            0 => zeros += 1,
            1 => ones += 1,
            _ => panic!("could not match bit")
        }
    }
    if zeros > ones {
        return (String::from("0"), String::from("1"));
    } else {
        return (String::from("1"), String::from("0"));
    }
}

fn main() {
    let input = common::read_new_line_input(3);
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for n in 0..input[0].len() {
        let (g, e) = resolve_bit(n, &input);
        gamma = format!("{gamma}{}", g);
        epsilon = format!("{epsilon}{}", e);
    }
    let gamma_dec = i32::from_str_radix(&gamma, 2).expect("should parse binary");
    let epsilon_dec = i32::from_str_radix(&epsilon, 2).expect("should parse binary");
    println!("{}", gamma_dec * epsilon_dec);
}