use std::{convert::TryInto, ops::Add, iter::FromIterator, collections::HashMap};

// Helpers

#[allow(dead_code)]
pub fn sort_string(input: String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    chars.sort_by(|a, b| a.cmp(b));
    return String::from_iter(chars);
}

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

#[allow(dead_code)]
pub fn exclude_string(target: &String, exclusion: &String) -> (String, String) {
    let mut remaining = String::new();
    let mut excluded = String::new();
    'outer: for i in 0..target.len() {
        let target_letter = target.get(i..i+1).unwrap();
        for j in 0..exclusion.len() {
            if target_letter == exclusion.get(j..j+1).unwrap() {
                excluded = excluded.add(target_letter);
                continue 'outer
            }
        }
        remaining = remaining.add(target_letter);
    }
    return (remaining, excluded);
}

#[allow(dead_code)]
pub fn transform_output(output: &str, mapping: &HashMap<String, String>) -> String {
    let mut result = String::new();
    for char in output.chars() {
        let token = mapping.get(&char.to_string()).expect("should have segment key");
        result = result.add(token);
    }
    return result;
}

// Structs

#[derive(Debug)]
pub struct Display {
    #[allow(dead_code)]
    signals: [String; 10],
    outputs: [String; 4]
}

impl Display {
    pub fn new(signals: [String;10], outputs: [String; 4]) -> Display {
        return Display{signals, outputs}
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn solve(&self) -> u16 {
        let mut undef_signals: Vec<String> = self.signals.iter().map(String::from).collect();
        let mut def_signals: [String; 10] = Default::default();
        let mut mapping: HashMap<String, String> = Default::default();
        // Filter useful signals
        let mut i = 0;
        while i < undef_signals.len() {
            let signal = &undef_signals[i];
            match signal.len() {
                2 => {def_signals[1] = undef_signals.remove(i);},
                3 => {def_signals[7] = undef_signals.remove(i);},
                4 => {def_signals[4] = undef_signals.remove(i);},
                7 => {def_signals[8] = undef_signals.remove(i);},
                6 => i += 1,
                _ => {undef_signals.remove(i);}
            }
        }
        // Find letter A
        let (a, _) = exclude_string(def_signals.get(7).unwrap(), def_signals.get(1).unwrap());
        mapping.insert(a, String::from("a"));
        // Find signal 6, letters C and F
        for i in 0..undef_signals.len() {
            let six_signal = &undef_signals[i];
            let (_, f) = exclude_string(six_signal, def_signals.get(1).unwrap());
            if f.len() != 1 {
                continue;
            }
            def_signals[6] = undef_signals.remove(i);
            mapping.insert(f.clone(), String::from("f"));
            let (c, _) = exclude_string(def_signals.get(1).unwrap(), &f);
            mapping.insert(c, String::from("c"));
            break;
        }
        // Find signal 0, letters B and D
        let (bd, _) = exclude_string(def_signals.get(4).unwrap(), def_signals.get(1).unwrap());
        for i in 0..undef_signals.len() {
            let zero_signal = &undef_signals[i];
            let (_, b) = exclude_string(zero_signal, &String::from(&bd));
            if b.len() != 1 {
                continue;
            }
            def_signals[0] = undef_signals.remove(i);
            mapping.insert(b.clone(), String::from("b"));
            let (d, _) = exclude_string(&bd, &b);
            mapping.insert(d, String::from("d"));
            break;
        }
        // Find signal 9, letters E and G
        let nine_signal = &undef_signals[0];
        let (e, _) = exclude_string(&String::from("abcdefg"), nine_signal);
        mapping.insert(e.clone(), String::from("e"));
        let mapping_keys = mapping.keys().map(|s| &**s).collect::<Vec<_>>().join("");
        let (g, _) = exclude_string(&String::from("abcdefg"), &mapping_keys);
        mapping.insert(g, String::from("g"));
        // Calculate result
        let mut result = String::new();
        for output in self.outputs.iter() {
            let trans_output = sort_string(transform_output(output, &mapping));
            match trans_output.as_str() {
                "abcefg" => result = result.add("0"),
                "cf" => result = result.add("1"),
                "acdeg" => result = result.add("2"),
                "acdfg" => result = result.add("3"),
                "bcdf" => result = result.add("4"),
                "abdfg" => result = result.add("5"),
                "abdefg" => result = result.add("6"),
                "acf" => result = result.add("7"),
                "abcdefg" => result = result.add("8"),
                "abcdfg" => result = result.add("9"),
                _ => continue
            }
        }
        return result.parse::<u16>().unwrap();
    }
}