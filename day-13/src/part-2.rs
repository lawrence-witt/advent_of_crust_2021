mod utils;

fn main() {
    let (mut points, folds, mut x, mut y) = utils::get_input();
    for f in folds {
        if f.axis == utils::Axis::X {
            x -= f.value + 1;
        } else {
            y = f.value
        }
        for p in points.iter_mut() {
            p.fold(&f);
        }
    }
    let unique_points = utils::get_unique_points(&points);
    let mut grid = vec![vec![String::from("."); (x + 1).into()]; (y + 1).into()];
    for p in &unique_points {
        let row: &mut Vec<String> = grid.get_mut(usize::from(p.y)).unwrap();
        let _ = std::mem::replace(&mut row[usize::from(p.x)], String::from("X"));
    }
    for mut row in grid {
        row.reverse();
        println!("{}", row.join(""));
    }
}