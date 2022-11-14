use common::*;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

fn get_next_day(members: &Vec<String>) -> String {
    let mut current = 1;
    for member in members {
        let exp = Regex::new(r"^day-(1?\d|2[1-5])$").unwrap();
        let test = exp.captures(&member);
        match test {
            Some(group) => {
                let day = group.get(1).unwrap().as_str();
                current = std::cmp::max(current, str_to_i32(day));
            }
            None => continue,
        }
    }
    if current == 25 {
        panic!("final entry 25 already exists")
    }
    return format!("day-{}", current + 1);
}

// Root Config

#[derive(Deserialize, Serialize)]
struct RootConfig {
    workspace: Members,
}

#[derive(Deserialize, Serialize)]
struct Members {
    members: Vec<String>,
}

fn read_root_config() -> RootConfig {
    let root_config_str =
        std::fs::read_to_string("Cargo.toml").expect("should return root Cargo file");
    return toml::from_str(&root_config_str).unwrap();
}

fn write_root_config(conf: &RootConfig) {
    let day_conf_str = toml::to_string_pretty(&conf).expect("should parse day config as string");
    std::fs::write("Cargo.toml", day_conf_str).expect("should write the day config");
}

// Day Config

#[derive(Deserialize, Serialize)]
struct DayConfig {
    package: Package,
    bin: Vec<Bin>,
    dependencies: Dependencies,
}

#[derive(Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Deserialize, Serialize)]
struct Bin {
    name: String,
    path: String,
}

#[derive(Deserialize, Serialize)]
struct Dependencies {
    common: Common,
}

#[derive(Deserialize, Serialize)]
struct Common {
    path: String,
}

fn write_day_config(location: &str, conf: &DayConfig) {
    let day_conf_location = format!("{location}/Cargo.toml");
    let day_conf_str = toml::to_string_pretty(&conf).expect("should parse day config as string");
    if std::path::Path::new(&day_conf_str).exists() {
        std::fs::remove_file(&day_conf_str).expect("should remove previous Cargo.toml");
    }
    std::fs::write(day_conf_location, day_conf_str).expect("should write the day config");
}

fn write_day(day: &str) {
    let day_src = format!("{}/src", &day);
    std::fs::create_dir_all(&day_src).expect("should create the day directory");
    std::fs::write(format!("{}/part-1.rs", &day_src), "").expect("should write part-1.rs");
    std::fs::write(format!("{}/part-2.rs", &day_src), "").expect("should write part-2.rs");
    let conf = DayConfig {
        package: Package {
            name: String::from(day),
            version: "0.1.0".to_string(),
        },
        bin: vec![
            Bin {
                name: "part-1".to_string(),
                path: "src/part-1.rs".to_string(),
            },
            Bin {
                name: "part-2".to_string(),
                path: "src/part-2.rs".to_string(),
            },
        ],
        dependencies: Dependencies {
            common: Common {
                path: "../common".to_string(),
            },
        },
    };
    write_day_config(&day, &conf);
}

// Script

fn main() {
    let mut conf = read_root_config();
    let next_day = get_next_day(&conf.workspace.members);
    write_day(&next_day);
    conf.workspace.members.push(String::from(&next_day));
    write_root_config(&conf)
}
