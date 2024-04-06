#[allow(dead_code)]
pub struct Stats {
    min: i32,
    max: i32,
    len: i32,
    mean: f64,
}

#[allow(dead_code)]
fn calc_stats(numbers: Vec<i32>) -> Stats {
    let mut cur_min: i32 = numbers[0];
    let mut cur_max: i32 = numbers[0];
    let mut length: i32 = 0;
    for num in numbers {
        length += 1;
        if num < cur_min {
            cur_min = num;
        } else if num > cur_max {
            cur_max = num;
        }
    }
    return Stats {
        min: cur_min,
        max: cur_max,
        len: length,
        mean: 0.0,
    };
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_is_1() {
        let numbers = vec![1, 2, 3, 4, 5];
        let stats = calc_stats(numbers);
        assert_eq!(stats.min, 1);
    }

    #[test]
    fn test_max_is_5() {
        let numbers = vec![1, 2, 3, 4, 5];
        let stats = calc_stats(numbers);
        assert_eq!(stats.max, 5);
    }

    #[test]
    fn test_len_is_5() {
        let numbers = vec![1, 2, 3, 4, 5];
        let stats = calc_stats(numbers);
        assert_eq!(stats.len, 5);
    }

    #[test]
    fn test_mean_is_3() {
        let numbers = vec![1, 2, 3, 4, 5];
        let stats = calc_stats(numbers);
        assert_eq!(stats.mean, 3.0);
    }
}

