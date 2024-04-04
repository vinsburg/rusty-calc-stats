#[allow(dead_code)]
pub struct Stats {
    min: i32,
    max: i32,
    len: i32,
    mean: f64,
}

#[allow(dead_code)]
fn calc_stats(numbers: Vec<i32>) -> Stats {
    return Stats {
        min: 0,
        max: 0,
        len: 0,
        mean: 0.0,
    };
}
