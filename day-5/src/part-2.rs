use std::{collections::HashMap, sync::{Arc, Mutex}};
extern crate common;
mod utils;

fn main() {
    let input = common::read_new_line_input(5);
    let lines = utils::format_input(input);
    let score_map: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let repeated = Arc::new(Mutex::new(0));
    for line in lines {
        line.for_point(&|point| {
            utils::score_point(point, &score_map, &repeated)
        });
    }
    println!("{}", repeated.lock().unwrap());
}