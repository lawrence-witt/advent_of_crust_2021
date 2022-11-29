pub fn get_input() -> (Vec<Point>, Vec<Fold>) {
    let input = common::read_input(13, "\n\n");
    let mut points: Vec<Point> = Default::default();
    let mut folds: Vec<Fold> = Default::default();
    for p in input.get(0).unwrap().split("\n").collect::<Vec<&str>>() {
        let xy: Vec<&str> = p.split(",").collect();
        points.push(Point{
            x: xy.get(0).unwrap().parse::<u16>().unwrap(), 
            y: xy.get(1).unwrap().parse::<u16>().unwrap()
        })
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
    return (points, folds)
}

#[derive(Debug)]
pub struct Point {
    x: u16,
    y: u16
}

impl Point {
    pub fn fold(&mut self, fold: &Fold) {
        if fold.axis == Axis::X {
            if self.x > fold.value {
                self.x -= fold.value
            } else {
                self.x = fold.value - self.x
            }
        } else {
            if self.y > fold.value {
                self.y -= fold.value
            } else {
                self.y = fold.value - self.y
            }
        }
    }

    pub fn to_string(&self) -> String {
        return format!("{}-{}", self.x, self.y);
    }
}

#[derive(Debug, PartialEq)]
pub enum Axis {
    X,
    Y
}

#[derive(Debug)]
pub struct Fold {
    axis: Axis,
    value: u16
}