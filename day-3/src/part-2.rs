extern crate common;

fn resolve_oxygen (zeros: Vec<String>, ones: Vec<String>) -> Vec<String> {
    return if zeros.len() > ones.len() {
        zeros
    } else {
        ones
    }
}

fn resolve_co2 (zeros: Vec<String>, ones: Vec<String>) -> Vec<String> {
    return if ones.len() < zeros.len() {
        ones
    } else {
        zeros
    }
}

fn resolve_frequency(i: usize, input: &Vec<String>, resolve: fn(zeros: Vec<String>, ones: Vec<String>) -> Vec<String>) -> String {
    let mut zeros: Vec<String> = vec![];
    let mut ones: Vec<String> = vec![];
    for num in input {
        match common::str_to_i32(&String::from(num.as_bytes()[i] as char)) {
            0 => zeros.push(String::from(num)),
            1 => ones.push(String::from(num)),
            _ => panic!("could not match bit")
        }
    }
    let result = &resolve(zeros, ones);
    if result.len() != 1 {
        return resolve_frequency(i + 1, result, resolve);
    }
    return String::from(&result[0 as usize]);
}

fn main() {
    let input = common::read_new_line_input(3);    
    let oxygen = resolve_frequency(0, &input, resolve_oxygen);
    let co2 = resolve_frequency(0, &input, resolve_co2);
    let oxygen_dec = i32::from_str_radix(&oxygen, 2).expect("should parse binary");
    let co2_dec = i32::from_str_radix(&co2, 2).expect("should parse binary");
    println!("{}", oxygen_dec * co2_dec);
}
