use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Finish,
};

use crate::game::{CubeColor, CubeSet, Game};

pub(crate) fn parse_input(input: &str) -> Vec<Game> {
    let (_, games) = game_list(input)
        .finish()
        .expect("Failed to parse puzzle input");

    games
}

fn integer(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn cube_color(input: &str) -> IResult<&str, CubeColor> {
    alt((
        map(tag("red"), |_| CubeColor::Red),
        map(tag("green"), |_| CubeColor::Green),
        map(tag("blue"), |_| CubeColor::Blue),
    ))(input)
}

fn cube_color_and_count(input: &str) -> IResult<&str, (u32, CubeColor)> {
    separated_pair(integer, space1, cube_color)(input)
}

fn cube_set(input: &str) -> IResult<&str, CubeSet> {
    map(separated_list1(tag(", "), cube_color_and_count), |cubes| {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for (count, color) in cubes {
            match color {
                CubeColor::Red => red += count,
                CubeColor::Green => green += count,
                CubeColor::Blue => blue += count,
            }
        }
        CubeSet::new(red, green, blue)
    })(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    map(
        tuple((
            preceded(tag("Game "), integer),
            preceded(tag(": "), separated_list1(tag("; "), cube_set)),
        )),
        |(id, sets)| Game::new(id, sets),
    )(input)
}

fn game_list(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cube_color() {
        let input = "red";

        let (rem, color) = cube_color(input).unwrap();

        assert_eq!(color, CubeColor::Red);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_cube_color_and_count() {
        let input = "5 blue";

        let (rem, (count, color)) = cube_color_and_count(input).unwrap();

        assert_eq!(count, 5);
        assert_eq!(color, CubeColor::Blue);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_cube_set() {
        let input = "1 blue, 2 green";

        let (rem, cube_set) = cube_set(input).unwrap();

        assert_eq!(cube_set.red, 0);
        assert_eq!(cube_set.blue, 1);
        assert_eq!(cube_set.green, 2);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let (rem, game) = game(input).unwrap();

        assert_eq!(game.id, 5);
        assert_eq!(game.sets.len(), 2);
        assert_eq!(game.sets[0].red, 6);
        assert_eq!(game.sets[0].green, 3);
        assert_eq!(game.sets[0].blue, 1);
        assert_eq!(game.sets[1].red, 1);
        assert_eq!(game.sets[1].green, 2);
        assert_eq!(game.sets[1].blue, 2);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_game_list() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";

        let (rem, games) = game_list(input).unwrap();

        assert_eq!(games.len(), 2);
        assert_eq!(games[0].id, 1);
        assert_eq!(games[0].sets.len(), 3);
        assert_eq!(games[0].sets[0].red, 4);
        assert_eq!(games[0].sets[0].green, 0);
        assert_eq!(games[0].sets[0].blue, 3);
        assert_eq!(games[0].sets[1].red, 1);
        assert_eq!(games[0].sets[1].green, 2);
        assert_eq!(games[0].sets[1].blue, 6);
        assert_eq!(games[0].sets[2].red, 0);
        assert_eq!(games[0].sets[2].green, 2);
        assert_eq!(games[0].sets[2].blue, 0);
        assert_eq!(games[1].id, 2);
        assert_eq!(games[1].sets.len(), 3);
        assert_eq!(games[1].sets[0].red, 0);
        assert_eq!(games[1].sets[0].green, 2);
        assert_eq!(games[1].sets[0].blue, 1);
        assert_eq!(games[1].sets[1].red, 1);
        assert_eq!(games[1].sets[1].green, 3);
        assert_eq!(games[1].sets[1].blue, 4);
        assert_eq!(games[1].sets[2].red, 0);
        assert_eq!(games[1].sets[2].green, 1);
        assert_eq!(games[1].sets[2].blue, 1);
        assert!(rem.is_empty());
    }
}
