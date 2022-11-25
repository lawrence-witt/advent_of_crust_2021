pub fn get_left_point(line_index: usize, value_index: usize, line: &String) -> Option<(usize, usize, u8)> {
    if value_index == 0 {
        return None
    }
    return Some((line_index, value_index - 1, line.get(value_index-1..value_index).unwrap().parse::<u8>().unwrap()));
}

pub fn get_right_point(line_index: usize, value_index: usize, line: &String) -> Option<(usize, usize, u8)> {
    if value_index == line.len() - 1 {
        return None
    }
    return Some((line_index, value_index + 1, line.get(value_index+1..value_index+2).unwrap().parse::<u8>().unwrap()));
}

pub fn get_above_point(line_index: usize, value_index: usize, lines: &Vec<String>) -> Option<(usize, usize, u8)> {
    if line_index == 0 {
        return None
    }
    return Some((line_index - 1, value_index, lines[line_index-1].get(value_index..value_index+1).unwrap().parse::<u8>().unwrap()));
}

pub fn get_below_point(line_index: usize, value_index: usize, lines: &Vec<String>) -> Option<(usize, usize, u8)> {
    if line_index == lines.len() - 1 {
        return None
    }
    return Some((line_index + 1, value_index, lines[line_index+1].get(value_index..value_index+1).unwrap().parse::<u8>().unwrap()));
}

pub fn value_is_lowpoint(value: u8, line_index: usize, value_index: usize, lines: &Vec<String>) -> bool {
    let line = lines.get(line_index).unwrap();
    if let Some((_, _, left_point)) = get_left_point(line_index, value_index, line) {
        if left_point <= value {
            return false
        }
    }
    if let Some((_, _, right_point)) = get_right_point(line_index, value_index, line) {
        if right_point <= value {
            return false
        }
    }
    if let Some((_, _, above_point)) = get_above_point(line_index, value_index, lines) {
        if above_point <= value {
            return false
        }
    }
    if let Some((_, _, below_point)) = get_below_point(line_index, value_index, lines) {
        if below_point <= value {
            return false
        }
    }
    return true
}