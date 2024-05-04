// main.rs
extern crate calc_stats;

use calc_stats::calc_stats;

fn main() {
    let numbers = vec![5, 2, 9, 1, 5, 6, 7, 8, 2, 3, 0, 4, 5, 6, 7, 8, 9];
    println!("The input sequence is:\n\t{:?}", numbers);
    let stats = calc_stats(numbers);
    println!(
        "The statistic properties are:\n\tMin: {}, Max: {}, Length: {}, Mean: {}",
        stats.min, stats.max, stats.len, stats.mean
    );
}
