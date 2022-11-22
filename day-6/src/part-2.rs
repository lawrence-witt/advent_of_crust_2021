extern crate common;
mod utils;

fn main() {
    let input = common::read_input(6, ",");
    let fishes = utils::format_input(input);
    let mut days: [usize; 9] = [0, 0, 0, 0, 0, 0, 0, 0, 0];
    for fish in fishes {
        days[usize::from(fish.days)] += 1;
    }
    let mut day = 256;
    while day > 0 {
        let spawned = days[0];
        days[0] = days[1];
        days[1] = days[2];
        days[2] = days[3];
        days[3] = days[4];
        days[4] = days[5];
        days[5] = days[6];
        days[6] = days[7] + spawned;
        days[7] = days[8];
        days[8] = spawned;
        day -= 1;
    }
    println!("{}", days.iter().sum::<usize>());
}