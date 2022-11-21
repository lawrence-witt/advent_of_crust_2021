use std::{collections::HashMap, sync::{Arc, Mutex}};
extern crate common;
mod utils;

fn main() {
    let input = common::read_new_line_input(5);
    let lines = utils::format_input(input);
    let score_map: Arc<Mutex<HashMap<String, i32>>> = Arc::new(Mutex::new(HashMap::new()));
    let points = Arc::new(Mutex::new(0));
    for line in lines {
        if line.is_diagonal() {
            continue;
        }
        line.for_point(&|point| {
            let key = point.to_string();
            let mut score_map_guard = score_map.lock().unwrap();
            let mut points_guard = points.lock().unwrap();
            let opt = &mut score_map_guard.get(&key);
            match opt {
                Some(value) => {
                    if **value == 1 {
                        *points_guard += 1;
                        score_map_guard.insert(key, 2);
                    }
                }
                None => {
                    score_map_guard.insert(key, 1);
                } 
            }
        });
    }
    println!("{}", points.lock().unwrap());
}