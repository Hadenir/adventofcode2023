mod galaxy_map;

use galaxy_map::*;
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let map: GalaxyMap = input.parse().unwrap();

    map.galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(galaxy1, galaxy2)| {
            let distance = galaxy1.distance(galaxy2);

            let min_x = galaxy1.x.min(galaxy2.x);
            let max_x = galaxy1.x.max(galaxy2.x);
            let min_y = galaxy1.y.min(galaxy2.y);
            let max_y = galaxy1.y.max(galaxy2.y);

            let empty_cols = map.empty_columns.iter().filter(|&&col| min_x < col && col < max_x).count();
            let empty_rows = map.empty_rows.iter().filter(|&&row| min_y < row && row < max_y).count();

            distance + empty_cols + empty_rows
        })
        .sum()
}

pub fn solve_part_2(input: &str, age_factor: usize) -> usize {
    let map: GalaxyMap = input.parse().unwrap();

    let add_cols_rows = age_factor - 1;
    map.galaxies
        .into_iter()
        .tuple_combinations()
        .map(|(galaxy1, galaxy2)| {
            let distance = galaxy1.distance(galaxy2);

            let min_x = galaxy1.x.min(galaxy2.x);
            let max_x = galaxy1.x.max(galaxy2.x);
            let min_y = galaxy1.y.min(galaxy2.y);
            let max_y = galaxy1.y.max(galaxy2.y);

            let empty_cols = map.empty_columns.iter().filter(|&&col| min_x < col && col < max_x).count();
            let empty_rows = map.empty_rows.iter().filter(|&&row| min_y < row && row < max_y).count();

            distance + empty_cols * add_cols_rows + empty_rows * add_cols_rows
        })
        .sum()
}
