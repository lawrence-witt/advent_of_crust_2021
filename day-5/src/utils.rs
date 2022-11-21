use std::{sync::{Arc, Mutex}, collections::HashMap};

extern crate regex;

// Helpers

fn step_to_target(value: i32, target: i32) -> i32 {
    if value < target {
        return value + 1
    } else if value > target {
        return value - 1
    } else {
        return value
    }
}

pub fn format_input(input: Vec<String>) -> Vec<Line> {
    let mut lines: Vec<Line> = Vec::new();
    let exp = regex::Regex::new(r"((\d+),(\d+))\s->\s((\d+),(\d+))\n?").unwrap();
    for line_input in input {
        let captures = exp.captures(&line_input).unwrap();
        let mut index = 0;
        let captures_len = captures.len();
        while index < captures_len {
            let x1 = common::str_to_i32(captures.get(index + 2).unwrap().as_str());
            let y1 = common::str_to_i32(captures.get(index + 3).unwrap().as_str());
            let x2 = common::str_to_i32(captures.get(index + 5).unwrap().as_str());
            let y2 = common::str_to_i32(captures.get(index + 6).unwrap().as_str());
            lines.push(Line::new(Point::new(x1, y1), Point::new(x2, y2)));
            index += 7;
        }
    }
    return lines;
}

pub fn score_point(point: &Point, score_map: &Arc<Mutex<HashMap<String, i32>>>, repeated: &Arc<Mutex<i32>>) {
    let key = point.to_string();
    let mut score_map_guard = score_map.lock().unwrap();
    let mut repeated_guard = repeated.lock().unwrap();
    let opt = &mut score_map_guard.get(&key);
    match opt {
        Some(value) => {
            if **value == 1 {
                *repeated_guard += 1;
                score_map_guard.insert(key, 2);
            }
        }
        None => {
            score_map_guard.insert(key, 1);
        } 
    }
}

// Structs

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point{x, y}
    }

    pub fn to_string(&self) -> String {
        return String::from(format!("{}-{}", self.x, self.y));
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        return Line {start, end}
    }

    #[allow(dead_code)]
    pub fn is_diagonal(&self) -> bool {
        return self.start.x != self.end.x && self.start.y != self.end.y;
    }

    pub fn for_point(&self, callback: &dyn Fn(&Point)) {
        fn for_point_recursive(prev: &Point, end: &Point, callback: &dyn Fn(&Point)) {
            callback(&prev);
            if prev.x == end.x && prev.y == end.y {
                return;
            }
            let next_x = step_to_target(prev.x, end.x);
            let next_y = step_to_target(prev.y, end.y);
            for_point_recursive(&Point::new(next_x, next_y), end, callback);
        }
        for_point_recursive(&self.start, &self.end, callback);
    }
}