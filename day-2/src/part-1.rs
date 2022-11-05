use common::*;

fn main() {
    let input = read_new_line_input(2);

    let mut horizontal = 0;
    let mut depth = 0;

    for i in input.iter() {
        let cmd: Vec<&str> = i.split_whitespace().collect();
        let dir = cmd[0];
        let v = str_to_i32(cmd[1]);
        match dir {
            "forward" => horizontal += v,
            "up" => depth -= v,
            "down" => depth += v,
            _ => panic!("invalid direction {}", dir)
        }
    }

    println!("{}", horizontal * depth);
}