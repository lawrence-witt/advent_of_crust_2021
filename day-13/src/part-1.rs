use std::collections::HashMap;

mod utils;

fn main() {
    let (mut points, folds) = utils::get_input();
    for p in points.iter_mut() {
        p.fold(&folds.get(0).unwrap());
    }
    let mut map: HashMap<String, i32> = Default::default();
    for p in points {
        let p_str = p.to_string();
        while None == map.get(&p_str) {
            map.insert(String::from(&p_str), 0);
        }
        let p_count = map.get(&p_str).unwrap();
        map.insert(String::from(&p_str), p_count + 1);
    }
    println!("{:#?}", map.keys().len());
}
