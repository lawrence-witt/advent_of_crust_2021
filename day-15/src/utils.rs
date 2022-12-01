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

#[derive(Debug, Clone)]
pub struct Mapping {
    pub current: (usize, usize),
    pub parent: (usize, usize),
    pub revalue: u32
}

#[derive(Debug)]
pub struct Node {
    pub parent: Option<(usize, usize)>,
    pub weight: u32,
    pub value: u32,
    pub is_visited: bool
}

impl Node {
    pub fn new(weight: u32) -> Node {
        return Node{parent: None, weight, value: u32::MAX, is_visited: false};
    }

    pub fn visit(&mut self, parent: (usize, usize), value: u32) {
        self.parent = Some(parent);
        self.value = self.revalue(value);
        self.is_visited = true;
    }

    pub fn revalue(&self, value: u32) -> u32 {
        return value + self.weight;
    }
}