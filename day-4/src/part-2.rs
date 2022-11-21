extern crate common;
mod utils;

fn main() {
    let input = common::read_new_line_input(4);
    let (numbers, mut boards) = utils::format_input(&input);
    for n in numbers {
        let mut index = 0;
        let mut boards_len = boards.len();
        while index < boards_len {
            let board = &mut boards[index];
            board.mark_cells(n);
            if board.is_complete() {
                if boards_len == 1 {
                    println!("{}", board.sum_unmarked() * n);
                    return;
                }
                boards.remove(index);
                boards_len -= 1;
                continue;
            }
            index += 1;
        }
    }
}