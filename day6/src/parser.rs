use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    Finish, IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<Race> {
    let (_, races) = races(input).finish().expect("Failed to parse puzzle input");

    races
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn number_list(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, number)(input)
}

fn times(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tuple((tag("Time:"), space1)), number_list)(input)
}

fn distances(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(tuple((tag("Distance:"), space1)), number_list)(input)
}

fn races(input: &str) -> IResult<&str, Vec<Race>> {
    map(
        separated_pair(times, line_ending, distances),
        |(times, distances)| {
            times
                .into_iter()
                .zip(distances)
                .map(|(time, record_distance)| Race {
                    time,
                    record_distance,
                })
                .collect()
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prase_number_list() {
        let input = "9  40  200";

        let (rem, numbers) = number_list(input).unwrap();

        assert_eq!(numbers, vec![9, 40, 200]);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_times() {
        let input = "Time:      7  15   30";

        let (rem, times) = times(input).unwrap();

        assert_eq!(times, vec![7, 15, 30]);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_distances() {
        let input = "Distance:  9  40  200";

        let (rem, distances) = distances(input).unwrap();

        assert_eq!(distances, vec![9, 40, 200]);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_races() {
        let input = "Time:      7  15
Distance:  9  40";

        let (rem, races) = races(input).unwrap();

        assert_eq!(races.len(), 2);
        assert_eq!(races[0].time, 7);
        assert_eq!(races[1].time, 15);
        assert_eq!(races[0].record_distance, 9);
        assert_eq!(races[1].record_distance, 40);
        assert!(rem.is_empty());
    }
}
