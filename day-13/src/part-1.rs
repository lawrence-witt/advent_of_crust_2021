mod utils;

fn main() {
    let (mut points, folds, _, _) = utils::get_input();
    for p in points.iter_mut() {
        p.fold(&folds.get(0).unwrap());
    }
    let unique_points = utils::get_unique_points(&points);
    println!("{:#?}", unique_points.len());
}
