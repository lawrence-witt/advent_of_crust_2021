// Helpers

use std::convert::TryInto;

pub fn format_input(input: Vec<String>) -> Vec<Display> {
    let mut displays: Vec<Display> = Vec::new();
    for i in input {
        let split_i: Vec<&str> = i.split(" | ").collect();
        let signals: [String; 10] = split_i[0].split(" ").map(String::from).collect::<Vec<String>>().try_into().expect("should collect signals");
        let outputs: [String; 4] = split_i[1].split(" ").map(String::from).collect::<Vec<String>>().try_into().expect("should collect outputs");
        displays.push(Display::new(signals, outputs))
    } 
    return displays;
}

// Structs

#[derive(Debug)]
pub struct Display {
    signals: [String; 10],
    outputs: [String; 4]
}

impl Display {
    pub fn new(signals: [String;10], outputs: [String; 4]) -> Display {
        return Display{signals, outputs}
    }

    pub fn count_unqiue(&self) -> u16 {
        let mut result = 0;
        for o in self.outputs.iter() {
            match o.len() {
                2 | 3 | 4 | 7 => result += 1,
                _ => continue
            }
        }
        return result;
    }
}