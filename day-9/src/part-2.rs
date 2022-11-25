use std::collections::HashMap;

extern crate common;
mod utils;

fn index_of_lowest(input: &Vec<usize>) -> Option<usize> {
    let mut lowest = input[0];
    let mut result = Some(0);
    for i in 0..input.len() {
        let curr = input[i];
        if curr < lowest {
            lowest = curr;
            result = Some(i);
        }
    }
    return result;
}

fn size_basin(line_index: usize, value_index: usize, lines: &Vec<String>) -> usize {
    let mut key_map: HashMap<String, bool> = Default::default();
    key_map.insert(format!("{line_index}-{value_index}"), true);
    fn size_basin_recursive(line_index: usize, value_index: usize, lines: &Vec<String>, map: &mut HashMap<String, bool>) {
        let line = lines.get(line_index).unwrap();
        let value = line.get(value_index..value_index+1).unwrap().parse::<u8>().unwrap();
        let left = utils::get_left_point(line_index, value_index, line);
        let right = utils::get_right_point(line_index, value_index, line);
        let above = utils::get_above_point(line_index, value_index, lines);
        let below = utils::get_below_point(line_index, value_index, lines);
        for node in [left, right, above, below] {
            if let Some((y, x, height)) = node {
                let key = format!("{y}-{x}");
                if height == 9 || map.contains_key(&key) {
                    continue
                }
                if height > value {
                    map.insert(key, true);
                    size_basin_recursive(y, x, lines, map);
                }
            }
        }
    }
    size_basin_recursive(line_index, value_index, lines, &mut key_map);
    return key_map.keys().len();
}

fn main() {
    let input = common::read_input(9, "\n");
    let mut highest: Vec<usize> = Vec::new();
    for i in 0..input.len() {
        let line = &input[i];
        for j in 0..line.len() {
            let value = line.get(j..j+1).unwrap().parse::<u8>().unwrap();
            if utils::value_is_lowpoint(value, i, j, &input) {
                let size = size_basin(i, j, &input);
                if highest.len() < 3 {
                    highest.push(size);
                } else {
                    let lowest_index = index_of_lowest(&highest).unwrap();
                    let lowest = highest.get(lowest_index).unwrap();
                    if *lowest < size {
                        highest[lowest_index] = size;
                    }
                }
            }
        };
    }
    println!("{:#?}", highest.iter().copied().reduce(|acc, curr| {
        return acc * curr
    }).expect("should reduce by multiplication"));
}