#![allow(clippy::complexity)]

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, preceded, terminated, tuple},
    Finish, IResult,
};

use crate::{almanac::*, range_map::RangeMap};

pub(crate) fn parse_input(
    input: &str,
) -> (
    Vec<Seed>,
    RangeMap<Seed, Soil>,
    RangeMap<Soil, Fertilizer>,
    RangeMap<Fertilizer, Water>,
    RangeMap<Water, Light>,
    RangeMap<Light, Temperature>,
    RangeMap<Temperature, Humidity>,
    RangeMap<Humidity, Location>,
) {
    let (_, almanac) = almanac(input)
        .finish()
        .expect("Failed to parse puzzle input");

    almanac
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn almanac_item<T: AlmanacItem>(input: &str) -> IResult<&str, T> {
    map(number, T::new)(input)
}

fn almanac_range<K: AlmanacItem, V: AlmanacItem>(input: &str) -> IResult<&str, (V, K, u64)> {
    tuple((
        terminated(almanac_item::<V>, space1),
        terminated(almanac_item::<K>, space1),
        number,
    ))(input)
}

fn seeds(input: &str) -> IResult<&str, Vec<Seed>> {
    preceded(
        tag("seeds: "),
        separated_list1(space1, almanac_item::<Seed>),
    )(input)
}

fn range_map<K: AlmanacItem, V: AlmanacItem>(input: &str) -> IResult<&str, RangeMap<K, V>> {
    map(
        separated_list1(line_ending, almanac_range::<K, V>),
        RangeMap::new,
    )(input)
}

fn almanac(
    input: &str,
) -> IResult<
    &str,
    (
        Vec<Seed>,
        RangeMap<Seed, Soil>,
        RangeMap<Soil, Fertilizer>,
        RangeMap<Fertilizer, Water>,
        RangeMap<Water, Light>,
        RangeMap<Light, Temperature>,
        RangeMap<Temperature, Humidity>,
        RangeMap<Humidity, Location>,
    ),
> {
    tuple((
        terminated(seeds, many1(line_ending)),
        delimited(
            tuple((tag("seed-to-soil map:"), line_ending)),
            range_map::<Seed, Soil>,
            many1(line_ending),
        ),
        delimited(
            tuple((tag("soil-to-fertilizer map:"), line_ending)),
            range_map::<Soil, Fertilizer>,
            many1(line_ending),
        ),
        delimited(
            tuple((tag("fertilizer-to-water map:"), line_ending)),
            range_map::<Fertilizer, Water>,
            many1(line_ending),
        ),
        delimited(
            tuple((tag("water-to-light map:"), line_ending)),
            range_map::<Water, Light>,
            many1(line_ending),
        ),
        delimited(
            tuple((tag("light-to-temperature map:"), line_ending)),
            range_map::<Light, Temperature>,
            many1(line_ending),
        ),
        delimited(
            tuple((tag("temperature-to-humidity map:"), line_ending)),
            range_map::<Temperature, Humidity>,
            many1(line_ending),
        ),
        delimited(
            tuple((tag("humidity-to-location map:"), line_ending)),
            range_map::<Humidity, Location>,
            many0(line_ending),
        ),
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_almanac_item() {
        let input = "50";

        let (rem, item) = almanac_item::<Soil>(input).unwrap();

        assert_eq!(item, Soil(50));
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_almanac_range() {
        let input = "50 98 2";

        let (rem, range) = almanac_range::<Water, Light>(input).unwrap();

        assert_eq!(range.0, Light(50));
        assert_eq!(range.1, Water(98));
        assert_eq!(range.2, 2);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_seeds() {
        let input = "seeds: 79 14 55 13";

        let (rem, seeds) = seeds(input).unwrap();

        assert_eq!(seeds, vec![Seed(79), Seed(14), Seed(55), Seed(13)]);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_range_map() {
        let input = "52 50 48";

        let (rem, range_map) = range_map::<Seed, Soil>(input).unwrap();

        assert_eq!(range_map.get(Seed(49)), Soil(49));
        assert_eq!(range_map.get(Seed(50)), Soil(52));
        assert_eq!(range_map.get(Seed(79)), Soil(81));
        assert_eq!(range_map.get(Seed(98)), Soil(98));
        assert!(rem.is_empty());
    }
}
