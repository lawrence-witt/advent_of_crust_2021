extern crate common;
mod utils;

fn main() {
    let input = common::read_new_line_input(4);
    let (numbers, mut boards) = utils::format_input(&input);
    for n in numbers {
        for b in boards.iter_mut() {
            b.mark_cells(n);
            if b.is_complete() {
                println!("{}", b.sum_unmarked() * n);
                return;
            }
        }
    }
}