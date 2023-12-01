mod parser;

use itertools::Itertools;
use parser::parse_input;

pub fn solve_part_1(input: &str) -> u32 {
    let lines = parse_input(input);

    lines
        .into_iter()
        .map(|line| {
            let first_digit = line.chars().find(char::is_ascii_digit).unwrap();
            let last_digit = line.chars().rev().find(char::is_ascii_digit).unwrap();
            let number_str = format!("{}{}", first_digit, last_digit);
            number_str.parse::<u32>().unwrap()
        })
        .sum()
}

const SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_first_digit(line: &str) -> char {
    let first_ascii_digit_opt = line.chars().find_position(char::is_ascii_digit);

    let fist_spelled_digit_opt = SPELLED_DIGITS
        .iter()
        .enumerate()
        .filter_map(|(digit_idx, spelled_digit)| {
            let match_result = line.match_indices(spelled_digit).next();

            match_result
                .map(|(idx, _)| (idx, char::from_digit((digit_idx + 1) as u32, 10).unwrap()))
        })
        .min_by_key(|&(idx, _)| idx);

    match (first_ascii_digit_opt, fist_spelled_digit_opt) {
        (Some((_, ch)), None) => ch,
        (None, Some((_, ch))) => ch,
        (Some((ascii_idx, ascii_char)), Some((spelled_idx, spelled_char))) => {
            if ascii_idx < spelled_idx {
                ascii_char
            } else {
                spelled_char
            }
        }
        _ => unreachable!(),
    }
}

fn get_last_digit(line: &str) -> char {
    let first_ascii_digit_opt = line
        .chars()
        .enumerate()
        .collect_vec()
        .into_iter()
        .rev()
        .find(|(_, ch)| ch.is_ascii_digit());

    let fist_spelled_digit_opt = SPELLED_DIGITS
        .iter()
        .enumerate()
        .filter_map(|(digit_idx, spelled_digit)| {
            let match_result = line.rmatch_indices(spelled_digit).next();

            match_result
                .map(|(idx, _)| (idx, char::from_digit((digit_idx + 1) as u32, 10).unwrap()))
        })
        .max_by_key(|&(idx, _)| idx);

    match (first_ascii_digit_opt, fist_spelled_digit_opt) {
        (Some((_, ch)), None) => ch,
        (None, Some((_, ch))) => ch,
        (Some((ascii_idx, ascii_char)), Some((spelled_idx, spelled_char))) => {
            if ascii_idx > spelled_idx {
                ascii_char
            } else {
                spelled_char
            }
        }
        _ => unreachable!(),
    }
}

pub fn solve_part_2(input: &str) -> u32 {
    let lines = parse_input(input);

    lines
        .into_iter()
        .map(|line| {
            let first_digit = get_first_digit(line);
            let last_digit = get_last_digit(line);

            let number_str = format!("{}{}", first_digit, last_digit);
            number_str.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit() {
        let line1 = "abc1def2ghi";
        assert_eq!(get_first_digit(line1), '1');

        let line2 = "abconed3eftwoghi";
        assert_eq!(get_first_digit(line2), '1');

        let line3 = "twone";
        assert_eq!(get_first_digit(line3), '2');

        assert_eq!(get_first_digit("two1nine"), '2');
        assert_eq!(get_first_digit("eightwothree"), '8');
        assert_eq!(get_first_digit("abcone2threexyz"), '1');
        assert_eq!(get_first_digit("xtwone3four"), '2');
        assert_eq!(get_first_digit("4nineeightseven2"), '4');
        assert_eq!(get_first_digit("zoneight234"), '1');
        assert_eq!(get_first_digit("7pqrstsixteen"), '7');
        assert_eq!(get_first_digit("eightwothree"), '8');
    }

    #[test]
    fn test_get_last_digit() {
        let line1 = "abc1def2ghi";
        assert_eq!(get_last_digit(line1), '2');

        let line2 = "abconed3eftwoghi";
        assert_eq!(get_last_digit(line2), '2');

        let line3 = "twone";
        assert_eq!(get_last_digit(line3), '1');

        assert_eq!(get_last_digit("two1nine"), '9');
        assert_eq!(get_last_digit("eightwothree"), '3');
        assert_eq!(get_last_digit("abcone2threexyz"), '3');
        assert_eq!(get_last_digit("xtwone3four"), '4');
        assert_eq!(get_last_digit("4nineeightseven2"), '2');
        assert_eq!(get_last_digit("zoneight234"), '4');
        assert_eq!(get_last_digit("7pqrstsixteen"), '6');
        assert_eq!(get_last_digit("eightwothree"), '3');
    }
}
