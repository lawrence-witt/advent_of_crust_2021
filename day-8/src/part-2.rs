extern crate common;
mod utils;

fn main() {
    let input = common::read_input(8, "\n");
    let displays = utils::format_input(input);
    let mut result: u64 = 0;
    for display in displays {
      result += u64::from(display.solve())
    }
    println!("{:#?}", result)
}