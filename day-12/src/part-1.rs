use std::collections::HashMap;
mod utils;

fn traverse(key: String, links: &HashMap<String, HashMap<String, bool>>, path: &mut Vec<String>, paths: &mut Vec<String>) {
    let options = links.get(&key).unwrap();
    for opt in options.keys() {
        let opt_as_str = opt.as_str();
        if opt_as_str == "start" {
            continue;
        }
        if opt_as_str == opt_as_str.to_lowercase() && utils::some(path, opt_as_str.to_string()) {
            continue;
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