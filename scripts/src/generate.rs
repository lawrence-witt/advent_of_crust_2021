use common::str_to_i32;
use glob::glob;
use std::cmp::max;
use std::error::Error;

fn get_next_day() -> Result<i32, Box<dyn Error>> {
    let mut current = 1;
    for entry in glob("day-*").expect("failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let segments: Vec<String> = path.display().to_string().split("-").map(String::from).collect();
                let day = str_to_i32(&segments[1]);
                current = max(current, day);
            },
            Err(e) => return Err(e)?,
        }
    }
    if current > 24 {
        return Err("day limit reached")?;
    }
    return Ok(current + 1);
}

fn main() {
    let next = get_next_day().expect("next day should be available");
    println!("{next}")
    // create new folder with /src/part-1.rs and /src/part-2.rs and Cargo.toml
    // add the new folder to the root Cargo.toml
}
