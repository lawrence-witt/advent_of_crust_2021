use std::fs;
use common::str_to_i32;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

fn get_current_day(members: Vec<String>) -> i32 {
    let mut current = 1;
    for member in members {
        let exp = Regex::new(r"^day-(1?\d|2[1-5])$").unwrap();
        let test = exp.captures(&member);
        match test {
            Some(group) => {
                let day = group.get(1).unwrap().as_str();
                current = std::cmp::max(current, str_to_i32(day));
            },
            None => continue
        }
    }
    return current;
}

#[derive(Deserialize, Serialize)]
struct RootConfig {
    workspace: Members
}

#[derive(Deserialize, Serialize)]
struct Members {
    members: Vec<String>
}

fn read_root_config() -> RootConfig {
    let root_config_str = fs::read_to_string("Cargo.toml").expect("should return root Cargo file");
    return toml::from_str(&root_config_str).unwrap();
}

fn main() {
    let conf = read_root_config();
    let current_day = get_current_day(conf.workspace.members);
    if current_day == 25 {
        panic!("final entry 25 already exists")
    }
    let next_day = current_day + 1;
}
