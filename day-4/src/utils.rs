pub fn every<T>(vector: &Vec<T>, predicate: &dyn Fn(&T) -> bool) -> bool {
    for v in vector {
        if predicate(v) == false {
            return false;
        }
    }
    return true;
}

pub fn format_input(input: &Vec<String>) -> (Vec<i32>, Vec<Board>) {
    let numbers:Vec<i32> = input[0].split(',').into_iter().map(common::str_to_i32).collect();
    let mut boards: Vec<Board> = vec![];
    let mut range = 1..input.len();
    while let Some(i) = range.next() {
        if i >= input.len() {
            continue;
        }
        let mut board = Board::new();
        let rows = &input[i+1..i+6];
        for j in 0..rows.len() {
            board.append_row(rows[j].split_whitespace().into_iter().map(Cell::new).collect());
        }
        boards.push(board);
        range = (i+6)..input.len();
    }
    return (numbers, boards);
}

#[derive(Debug)]
pub struct Board {
    pub rows: Vec<Vec<Cell>>
}

impl Board {
    fn new() -> Board {
        Board { rows: vec![] }
    }

    pub fn append_row(& mut self, row: Vec<Cell>) {
        self.rows.push(row);
    }

    pub fn mark_cells(& mut self, value: i32) {
        for row in self.rows.iter_mut() {
            for cell in row {
                if cell.value == value {
                    cell.mark();
                    return;
                }
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        for i in 0..self.rows.len() {
            let row_complete = every(&self.rows[i], &|cell: &Cell| {
                return cell.marked;
            });
            if row_complete {
                return true;
            }
            let col_complete = every(&self.rows, &|row: &Vec<Cell>| {
                return row[i].marked;
            });
            if col_complete {
                return true;
            }
        }
        return false;
    }

    pub fn sum_unmarked(& self) -> i32 {
        let mut result = 0;
        for row in self.rows.iter() {
            for cell in row {
                if cell.marked {
                    continue;
                }
                result += cell.value
            }
        }
        return result;
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Cell {
    value: i32,
    marked: bool
}

impl Cell {
    pub fn new(value: &str) -> Cell {
        Cell { value: common::str_to_i32(value), marked: false }
    }

    pub fn mark(& mut self) {
        self.marked = true;
    }
}