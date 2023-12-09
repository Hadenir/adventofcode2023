mod parser;

use itertools::Itertools;
use parser::parse_input;

fn predict_next_value(values: &[i64]) -> i64 {
    assert!(!values.is_empty());

    if values.iter().all(|&v| v == 0) {
        return 0;
    }

    let diffs = values.windows(2).map(|w| w[1] - w[0]).collect_vec();
    values.last().unwrap() + predict_next_value(&diffs)
}

pub fn solve_part_1(input: &str) -> i64 {
    let report = parse_input(input);

    report
        .into_iter()
        .map(|history| predict_next_value(&history))
        .sum()
}

fn predict_previous_value(values: &[i64]) -> i64 {
    assert!(!values.is_empty());

    if values.iter().all(|&v| v == 0) {
        return 0;
    }

    let diffs = values.windows(2).map(|w| w[1] - w[0]).collect_vec();
    values.first().unwrap() - predict_previous_value(&diffs)
}

pub fn solve_part_2(input: &str) -> i64 {
    let report = parse_input(input);

    report
        .into_iter()
        .map(|history| predict_previous_value(&history))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_next_value() {
        assert_eq!(predict_next_value(&[0, 0, 0]), 0);
        assert_eq!(predict_next_value(&[3, 3, 3, 3]), 3);
        assert_eq!(predict_next_value(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(predict_next_value(&[1, 3, 6 ,10, 15, 21]), 28);
        assert_eq!(predict_next_value(&[10, 13, 16, 21, 30, 45]), 68);
    }

    #[test]
    fn test_predict_previous_value() {
        assert_eq!(predict_previous_value(&[0, 0, 0]), 0);
        assert_eq!(predict_previous_value(&[3, 3, 3, 3]), 3);
        assert_eq!(predict_previous_value(&[0, 3, 6, 9, 12, 15]), -3);
        assert_eq!(predict_previous_value(&[1, 3, 6 ,10, 15, 21]), 0);
        assert_eq!(predict_previous_value(&[10, 13, 16, 21, 30, 45]), 5);
    }
}
