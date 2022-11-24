extern crate common;
mod utils;

fn main() {
    let input = common::read_input(7, "\n");
    let displays = utils::format_input(input);
    let mut result = 0;
    for display in displays {
      result += display.count_unqiue()
    }
    println!("{:#?}", result)
}