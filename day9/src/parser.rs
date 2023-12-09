use nom::{
    character::complete::{char, digit1, space1, line_ending},
    combinator::{map_res, opt},
    multi::separated_list1,
    sequence::tuple,
    IResult, Finish,
};

pub(crate) fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let (_, report) = oasis_report(input)
        .finish()
        .expect("Failed to parse puzzle input");

    report
}

fn number(input: &str) -> IResult<&str, i64> {
    map_res(tuple((opt(char('-')), digit1)), |(sign, number)| {
        format!("{}{}", sign.unwrap_or('+'), number).parse()
    })(input)
}

fn value_history(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, number)(input)
}

fn oasis_report(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    separated_list1(line_ending, value_history)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_positive_number() {
        let input = "123";

        let (rem, number) = number(input).unwrap();

        assert_eq!(number, 123);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_negative_number() {
        let input = "-123";

        let (rem, number) = number(input).unwrap();

        assert_eq!(number, -123);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_value_history() {
        let input = "0 3 6 -9 12 15";

        let (rem, history) = value_history(input).unwrap();

        assert_eq!(history, vec![0, 3, 6, -9, 12, 15]);
        assert!(rem.is_empty());
    }
}
