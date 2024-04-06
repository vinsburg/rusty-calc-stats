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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        let numbers = vec![1, 2, 3, 4, 5];
        let stats = calc_stats(numbers);
        assert_eq!(stats.min, 1);
    }
}

