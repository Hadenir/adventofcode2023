#![allow(clippy::type_complexity)]

use nom::{
    character::complete::{char, digit1, line_ending, one_of, space1},
    combinator::{map, map_res},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<Record> {
    let (_, records) = records_list(input)
        .finish()
        .expect("Failed to parse puzzle input");

    records
}

fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn spring(input: &str) -> IResult<&str, Spring> {
    map_res(one_of(".#?"), Spring::try_from)(input)
}

fn springs_record(input: &str) -> IResult<&str, Vec<Spring>> {
    many1(spring)(input)
}

fn groups_record(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(char(','), number)(input)
}

fn record(input: &str) -> IResult<&str, Record> {
    map(
        separated_pair(springs_record, space1, groups_record),
        |(springs, groups)| Record { springs, groups },
    )(input)
}

fn records_list(input: &str) -> IResult<&str, Vec<Record>> {
    separated_list1(line_ending, record)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_springs_record() {
        let input = "???.###";

        let (rem, springs) = springs_record(input).unwrap();

        assert_eq!(
            springs,
            vec![
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged
            ]
        );
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_groups_record() {
        let input = "1,1,3";

        let (rem, groups) = groups_record(input).unwrap();

        assert_eq!(groups, vec![1, 1, 3]);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_record() {
        let input = "#.?.#? 1,1,2";

        let (rem, record) = record(input).unwrap();

        assert_eq!(
            record.springs,
            vec![
                Spring::Damaged,
                Spring::Operational,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Unknown
            ]
        );
        assert_eq!(record.groups, vec![1, 1, 2]);
        assert!(rem.is_empty());
    }
}
