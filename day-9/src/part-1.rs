extern crate common;

fn main() {
    let input = common::read_input(9, "\n");
    let mut result: u32 = 0;
    for i in 0..input.len() {
        let line = &input[i];
        'line: for j in 0..line.len() {
            let value = line.get(j..j+1).unwrap().parse::<u8>().unwrap();
            let left_height = if j == 0 {
                None
            } else {
                Some(line.get(j-1..j).unwrap().parse::<u8>().unwrap())
            };
            let right_height = if j == line.len() - 1 {
                None
            } else {
                Some(line.get(j+1..j+2).unwrap().parse::<u8>().unwrap())
            };
            let above_height = if i == 0 {
                None
            } else {
                let above_line = &input[i-1];
                Some(above_line.get(j..j+1).unwrap().parse::<u8>().unwrap())
            };
            let below_height = if i == input.len() - 1 {
                None
            } else {
                let below_line = &input[i+1];
                Some(below_line.get(j..j+1).unwrap().parse::<u8>().unwrap())
            };
            let sibling_heights = [left_height, right_height, above_height, below_height];
            for sibling_height in sibling_heights {
                match sibling_height {
                    Some(h) => {
                        if h <= value {
                            continue 'line
                        }
                    },
                    None => continue
                }
            }
            result += 1 + u32::from(value);
        };
    }
    println!("{:#?}", result);
}