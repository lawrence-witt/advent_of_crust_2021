mod utils;

fn main() {
    let mut input = utils::get_input();
    utils::solve(&mut input);
    println!("{}", input.last().unwrap().last().unwrap().value);
}