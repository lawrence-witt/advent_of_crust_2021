use common;

struct Direction {
    horizontal: i32,
    depth: i32
}

impl Direction {
    fn new() -> Direction {
        return Direction{horizontal: 0, depth: 0}
    }
    fn forward(&mut self, value: i32) {
        self.horizontal += value
    }
    fn up(&mut self, value: i32) {
        self.depth -= value;
    }
    fn down(&mut self, value: i32) {
        self.depth += value;
    }
}

fn main() {
    let input = common::read_new_line_input(2);

    let mut d = Direction::new();

    println!("{:?}", input);
}