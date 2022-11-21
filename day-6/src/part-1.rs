extern crate common;
mod utils;

fn main() {
    let input = common::read_input(6, ",");
    let mut fishes = utils::format_input(input);
    let mut days = 80;
    while days != 0 {
        let mut spawned: Vec<utils::Fish> = Vec::new();
        for fish in fishes.iter_mut() {
            let result = fish.tick();
            match result {
                Some(spawn) => {
                    spawned.push(spawn);
                }
                None => continue
            }
        }
        fishes.append(&mut spawned);
        days -= 1;
    }
    println!("{}", fishes.len());
}
