use std::collections::HashMap;
mod utils;

fn count_duplicate_lowercases(vector: &Vec<String>) -> i32 {
    let mut map: HashMap<String, u32> = Default::default();
    for i in vector {
        if i.as_str() == i.to_lowercase() {
            while None == map.get(i) {
                map.insert(i.to_string(), 0);
            }
            let value = map.get(i).unwrap();
            map.insert(i.to_string(), value + 1);
        }
    }
    let mut count = 0;
    for (_, v) in map {
        if v == 2 {
            count += 1;
        }
    }
    return count;
}

fn traverse(key: String, links: &HashMap<String, HashMap<String, bool>>, path: &mut Vec<String>, paths: &mut Vec<String>) {
    let options = links.get(&key).unwrap();
    for opt in options.keys() {
        let opt_as_str = opt.as_str();
        if opt_as_str == "start" {
            continue;
        }
        if opt_as_str == opt_as_str.to_lowercase() {
            if utils::some(path, opt_as_str.to_string()) {
                if count_duplicate_lowercases(path) > 0 {
                    continue;
                }
            }
        }
        let mut next_path = path.clone();
        next_path.push(String::from(opt_as_str));
        if opt_as_str == "end" {
            let result = next_path.join(", ");
            paths.push(result);
            continue;
        }
        traverse(opt.to_string(), links, &mut next_path, paths)
    }
}

fn main() {
    let input = common::read_input(12, "\n");
    let formatted = utils::format_input(input);
    let mut paths : Vec<String> = Default::default();
    traverse(String::from("start"), &formatted, &mut vec![String::from("start")], &mut paths);
    println!("{}", paths.len());
}