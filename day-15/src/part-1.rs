use utils::Mapping;
mod utils;

fn get_adjacent_unvisited(x: &usize, y: &usize, input: &Vec<Vec<utils::Node>>) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Default::default();
    if *y != 0 {
        let top = input.get(y - 1).unwrap().get(*x).unwrap();
        if !top.is_visited {
            result.push((*x, y - 1));
        }
    }
    if *y != input.len() - 1 {
        let bottom = input.get(y + 1).unwrap().get(*x).unwrap();
        if !bottom.is_visited {
            result.push((*x, y + 1));
        }
    }
    let row = input.get(*y).unwrap();
    if *x != 0 {
        let left = row.get(x - 1).unwrap();
        if !left.is_visited {
            result.push((x - 1, *y));
        }
    }
    if *x != row.len() - 1 {
        let right = row.get(x + 1).unwrap();
        if !right.is_visited {
            result.push((x + 1, *y));
        }
    }
    return result;
}

fn get_best_mapping(mappings: &Vec<utils::Mapping>) -> Option<&utils::Mapping> {
    let mut best_value = u32::MAX;
    let mut best_mapping = mappings.get(0);
    for mapping in mappings {
        if mapping.revalue < best_value {
            best_value = mapping.revalue;
            best_mapping = Some(mapping);
        }
    }
    return best_mapping;
}

fn get_node_value(x: usize, y: usize, input: &mut Vec<Vec<utils::Node>>) -> u32 {
    return input.get_mut(y).unwrap().get_mut(x).unwrap().value;
}

fn main() {
    let mut input = utils::get_input();
    let first = input.get_mut(0).unwrap().get_mut(0).unwrap();
    first.weight = 0;
    first.value = 0;
    first.is_visited = true;
    let mut visited: Vec<(usize, usize)> = vec![(0, 0)];
    // until we visit the last node
    while !input.last().unwrap().last().unwrap().is_visited {
        let mut mappings: Vec<utils::Mapping> = Default::default();
        let mut i = 0;
        // loop over the visited nodes
        while i < visited.len() {
            let (px, py) = visited.get(i).unwrap();
            // get each visited node's unvisited neighbours
            let adjacent_unvisited = get_adjacent_unvisited(px, py, &input);
            // if none left, don't need to check it anymore
            if adjacent_unvisited.len() == 0 {
                visited.remove(i);
                continue;
            }
            // for each neighbour, create a mapping between its parent and potential new value
            for (cx, cy) in adjacent_unvisited {
                let child = input.get(cy).unwrap().get(cx).unwrap();
                let parent = input.get(py.to_owned()).unwrap().get(px.to_owned()).unwrap();
                mappings.push(Mapping{parent: (px.to_owned(), py.to_owned()), current: (cx, cy), revalue: child.revalue(parent.value)})
            }
            i += 1;
        }
        // get the least expensive neighbour and visit it, locking in its new value
        if let Some(m) = get_best_mapping(&mappings) {
            let parent_value = get_node_value(m.parent.0, m.parent.1, &mut input);
            let child = input.get_mut(m.current.1).unwrap().get_mut(m.current.0).unwrap();
            child.visit(m.parent, parent_value);
            visited.push(m.current);
        }
    }
    println!("{}", input.last().unwrap().last().unwrap().value);
}