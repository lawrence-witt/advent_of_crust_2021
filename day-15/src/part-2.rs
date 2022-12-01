mod utils;

fn expand(input: &mut Vec<Vec<utils::Node>>) -> Vec<Vec<utils::Node>> {
    let mut expanded: [[Vec<Vec<utils::Node>>; 5]; 5] = Default::default();

    for i in 0..5 {
        for j in 0..5 {
            let mut clone = input.clone();
            for r in clone.iter_mut() {
                for n in r.iter_mut() {
                    let new_weight = (n.weight + i + j) % 10;
                    n.weight = if new_weight < n.weight {
                        new_weight + 1
                    } else {
                        new_weight
                    }
                }
            }
            expanded[i as usize][j as usize] = clone;
        }
    }

    let mut result: Vec<Vec<utils::Node>> = Default::default();
    
    for expanded_row in expanded {
        let mut combined_row: Vec<Vec<utils::Node>> = Default::default();
        for _ in 0..expanded_row[0].len() {
            combined_row.push(vec![]);
        }
        for column in expanded_row {
            for j in 0..column.len() {
                let mut row = column.get(j).unwrap().to_owned();
                combined_row.get_mut(j).unwrap().append(&mut row);
            }
        }
        result.append(&mut combined_row);
    }

    return result;
}

fn main() {
    let mut input = utils::get_input();
    let mut expanded = expand(&mut input);
    utils::solve(&mut expanded);
    println!("{}", expanded.last().unwrap().last().unwrap().value);
}