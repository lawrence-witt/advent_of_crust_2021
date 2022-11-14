extern crate common;

fn main() {
    let input = common::read_new_line_input(2);
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for i in input.iter() {
        let cmd: Vec<&str> = i.split_whitespace().collect();
        let dir = cmd[0];
        let v = common::str_to_i32(cmd[1]);
        match dir {
            "forward" => {
                horizontal += v;
                depth += v * aim;
            }
            "up" => aim -= v,
            "down" => aim += v,
            _ => panic!("invalid direction {}", dir),
        }
    }
    println!("{}", horizontal * depth);
}
