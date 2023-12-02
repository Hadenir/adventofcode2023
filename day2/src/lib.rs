mod game;
mod parser;

use game::CubeSet;
use parser::parse_input;

fn check_cube_count(set: &CubeSet, red: u32, green: u32, blue: u32) -> bool {
    set.red <= red && set.green <= green && set.blue <= blue
}

pub fn solve_part_1(input: &str) -> u32 {
    let games = parse_input(input);

    games
        .into_iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| check_cube_count(set, 12, 13, 14))
        })
        .map(|game| game.id)
        .sum()
}

pub fn solve_part_2(input: &str) -> u32 {
    let games = parse_input(input);

    games.into_iter()
        .map(|game| {
            let mut minimal_set = CubeSet::new(0, 0, 0);
            for set in game.sets {
                minimal_set.red = minimal_set.red.max(set.red);
                minimal_set.green = minimal_set.green.max(set.green);
                minimal_set.blue = minimal_set.blue.max(set.blue);
            }
            minimal_set.red * minimal_set.green * minimal_set.blue
        })
        .sum()
}
