pub fn get_input() -> Vec<Vec<Node>> {
    let input = common::read_input(15, "\n");
    let mut rows: Vec<Vec<Node>> = Default::default();
    for row in input {
        rows.push(row.split("").filter_map(|v| {
            if v.is_empty() {
                return None;
            }
            return Some(Node::new(v.parse::<u32>().unwrap()))
        }).collect());
    }
    return rows;
}

/* Solve */

fn get_adjacent_unvisited(x: &usize, y: &usize, input: &Vec<Vec<Node>>) -> Vec<Point> {
    let mut result: Vec<Point> = Default::default();
    if *y != 0 {
        let top = get_node_at_point(*x, y - 1, input);
        if !top.is_visited {
            result.push(Point::new(*x, y - 1));
        }
    }
    if *y != input.len() - 1 {
        let bottom = get_node_at_point(*x, y + 1, input);
        if !bottom.is_visited {
            result.push(Point::new(*x, y + 1));
        }
    }
    if *x != 0 {
        let left = get_node_at_point(x - 1, *y, input);
        if !left.is_visited {
            result.push(Point::new(x - 1, *y));
        }
    }
    if *x != input.get(0).unwrap().len() - 1 {
        let right = get_node_at_point(x + 1, *y, input);
        if !right.is_visited {
            result.push(Point::new(x + 1, *y));
        }
    }
    return result;
}

fn get_best_mapping(mappings: &Vec<Mapping>) -> Option<&Mapping> {
    let mut best_value = u32::MAX;
    let mut best_mapping = mappings.get(0);
    for mapping in mappings {
        if mapping.valuation < best_value {
            best_value = mapping.valuation;
            best_mapping = Some(mapping);
        }
    }
    return best_mapping;
}

fn get_mut_node_at_point(x: usize, y: usize, input: &mut Vec<Vec<Node>>) -> &mut Node {
    return input.get_mut(y).unwrap().get_mut(x).unwrap();
}

fn get_node_at_point(x: usize, y: usize, input: &Vec<Vec<Node>>) -> &Node {
    return input.get(y).unwrap().get(x).unwrap();
}

pub fn solve(input: &mut Vec<Vec<Node>>) {
    let first = input.get_mut(0).unwrap().get_mut(0).unwrap();
    first.weight = 0;
    first.value = 0;
    first.is_visited = true;
    let mut visited: Vec<Point> = vec![Point::new(0, 0)];
    // until we visit the last node
    while !input.last().unwrap().last().unwrap().is_visited {
        let mut mappings: Vec<Mapping> = Default::default();
        let mut i = 0;
        // loop over the visited nodes
        while i < visited.len() {
            let parent_point = visited.get(i).unwrap();
            // get each visited node's unvisited neighbours
            let adjacent_unvisited = get_adjacent_unvisited(&parent_point.x, &parent_point.y, &input);
            // if none left, don't need to check it anymore
            if adjacent_unvisited.len() == 0 {
                visited.remove(i);
                continue;
            }
            // for each neighbour, create a mapping between its parent and potential new value
            for child_point in adjacent_unvisited {
                let child = get_node_at_point(child_point.x, child_point.y, input);
                let parent = get_node_at_point(parent_point.x, parent_point.y, input);
                mappings.push(Mapping::new(child_point, parent_point.clone(), child.evaluate(parent.value)));
            }
            i += 1;
        }
        // get the least expensive neighbour and visit it, locking in its new value
        if let Some(m) = get_best_mapping(&mappings) {
            let parent_value = get_node_at_point(m.parent.x, m.parent.y, input).value;  
            let child = get_mut_node_at_point(m.child.x, m.child.y, input);
            child.visit(parent_value);
            visited.push(m.child.clone());
        }
    }
}

/* Structs */

#[derive(Debug, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        return Point{x, y}
    }
}

#[derive(Debug, Clone)]
pub struct Mapping {
    pub child: Point,
    pub parent: Point,
    pub valuation: u32
}

impl Mapping {
    pub fn new(child: Point, parent: Point, valuation: u32) -> Mapping {
        return Mapping{child, parent, valuation}
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub weight: u32,
    pub value: u32,
    pub is_visited: bool
}

impl Node {
    pub fn new(weight: u32) -> Node {
        return Node{weight, value: u32::MAX, is_visited: false};
    }

    pub fn visit(&mut self, parent_value: u32) {
        self.value = self.evaluate(parent_value);
        self.is_visited = true;
    }

    pub fn evaluate(&self, value: u32) -> u32 {
        return value + self.weight;
    }
}