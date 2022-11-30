use std::collections::HashMap;

pub fn get_input() -> (Vec<Point>, Vec<Fold>, u16, u16) {
    let input = common::read_input(13, "\n\n");
    let mut points: Vec<Point> = Default::default();
    let mut folds: Vec<Fold> = Default::default();
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    for p in input.get(0).unwrap().split("\n").collect::<Vec<&str>>() {
        let pxy: Vec<&str> = p.split(",").collect();
        let px = pxy.get(0).unwrap().parse::<u16>().unwrap();
        let py = pxy.get(1).unwrap().parse::<u16>().unwrap();
        x = std::cmp::max(x, px);
        y = std::cmp::max(y, py);
        points.push(Point{ x: px, y: py })
    }
    let exp = regex::Regex::new(r"(?P<axis>x|y)=(?P<value>\d+)").unwrap();
    for f in input.get(1).unwrap().split("\n").collect::<Vec<&str>>() {
        for caps in exp.captures_iter(f) {
            let axis =  if &caps["axis"] == "x" {
                Axis::X
            } else {
                Axis::Y
            };
            let value = &caps["value"].parse::<u16>().unwrap();
            folds.push(Fold { axis, value: u16::from(*value) })
        }
    }
    return (points, folds, x, y)
}

pub fn get_unique_points(points: &Vec<Point>) -> Vec<Point> {
    let mut map: HashMap<String, Point> = Default::default();
    for p in points {
        let p_str = p.to_string();
        if None == map.get(&p_str) {
            map.insert(String::from(&p_str), p.clone());
        }
    }
    return map.into_values().collect();
}

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16
}

impl Point {
    pub fn fold(&mut self, fold: &Fold) {
        if fold.axis == Axis::X {
            if self.x > fold.value {
                self.x -= fold.value + 1;
            } else {
                self.x = fold.value - self.x - 1;
            }
        } else {
            if self.y > fold.value {
                self.y = fold.value - (self.y - fold.value);
            }
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}-{}", self.x, self.y);
    }

    pub fn clone(&self) -> Point {
        return Point{x: self.x, y: self.y}
    }
}

#[derive(Debug, PartialEq)]
pub enum Axis {
    X,
    Y
}

#[derive(Debug)]
pub struct Fold {
    pub axis: Axis,
    pub value: u16
}