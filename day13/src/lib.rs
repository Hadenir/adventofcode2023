use std::{collections::BTreeSet, str::FromStr};

use grid::Grid;
use itertools::Itertools;

pub mod grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        match char {
            '.' => Ok(Tile::Ash),
            '#' => Ok(Tile::Rock),
            _ => Err("Invalid character"),
        }
    }
}

struct Map {
    patterns: Vec<Grid<Tile>>,
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let patterns = input
            .replace("\r\n", "\n")
            .split("\n\n")
            .map(|pattern_str| {
                let lines = pattern_str.lines().collect_vec();
                let height = lines.len();
                let width = lines[0].len();
                let tiles: Vec<_> = lines
                    .into_iter()
                    .flat_map(|line| line.chars())
                    .map(TryFrom::try_from)
                    .collect::<Result<_, _>>()?;

                Ok(Grid::new(width, height, tiles))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self { patterns })
    }
}

fn find_vertical_reflection_points(grid: &Grid<Tile>) -> BTreeSet<usize> {
    (0..grid.height)
        .map(|y| {
            (0..(grid.width - 1))
                .filter(|&x| {
                    let left = (0..=x).rev();
                    let right = (x + 1)..grid.width;

                    left.zip(right)
                        .all(|(l, r)| grid.get(l, y) == grid.get(r, y))
                })
                .collect::<BTreeSet<_>>()
        })
        .tree_fold1(|a, b| a.intersection(&b).cloned().collect())
        .expect("grid is not empty")
}

fn find_horizontal_reflection_points(grid: &Grid<Tile>) -> BTreeSet<usize> {
    (0..grid.width)
        .map(|x| {
            (0..(grid.height - 1))
                .filter(|&y| {
                    let up = (0..=y).rev();
                    let down = (y + 1)..grid.height;

                    up.zip(down).all(|(u, d)| grid.get(x, u) == grid.get(x, d))
                })
                .collect::<BTreeSet<_>>()
        })
        .tree_fold1(|a, b| a.intersection(&b).cloned().collect())
        .expect("grid is not empty")
}

fn find_reflection(grid: Grid<Tile>) -> usize {
    if let Some(x) = find_vertical_reflection_points(&grid).into_iter().next() {
        x + 1
    } else if let Some(x) = find_horizontal_reflection_points(&grid).into_iter().next() {
        100 * (x + 1)
    } else {
        panic!("Pattern contains no reflections!")
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    map.patterns.into_iter().map(find_reflection).sum()
}

fn find_vertical_reflection_points_with_smudge(grid: &Grid<Tile>) -> BTreeSet<usize> {
    (0..grid.height)
        .flat_map(|y: usize| {
            (0..(grid.width - 1)).filter(move |&x| {
                let left = (0..=x).rev();
                let right = (x + 1)..grid.width;

                left.zip(right)
                    .all(|(l, r)| grid.get(l, y) == grid.get(r, y))
            })
        })
        .counts()
        .into_iter()
        .filter_map(|(x, count)| (count == grid.height - 1).then_some(x))
        .collect()
}

fn find_horizontal_reflection_points_with_smudge(grid: &Grid<Tile>) -> BTreeSet<usize> {
    (0..grid.width)
        .flat_map(|x| {
            (0..(grid.height - 1)).filter(move |&y| {
                let up = (0..=y).rev();
                let down = (y + 1)..grid.height;

                up.zip(down).all(|(u, d)| grid.get(x, u) == grid.get(x, d))
            })
        })
        .counts()
        .into_iter()
        .filter_map(|(x, count)| (count == grid.width - 1).then_some(x))
        .collect()
}

fn find_reflection_with_smudge(grid: Grid<Tile>) -> usize {
    if let Some(x) = find_vertical_reflection_points_with_smudge(&grid).into_iter().next() {
        x + 1
    } else if let Some(x) = find_horizontal_reflection_points_with_smudge(&grid).into_iter().next() {
        100 * (x + 1)
    } else {
        panic!("Pattern contains no reflections!")
    }
}

pub fn solve_part_2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    map.patterns.into_iter().map(find_reflection_with_smudge).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pattern() {
        let input = ".##.
#..#
.#.#";

        let map: Map = input.parse().unwrap();

        assert_eq!(map.patterns.len(), 1);
        assert_eq!(map.patterns[0].width, 4);
        assert_eq!(map.patterns[0].height, 3);
        assert_eq!(map.patterns[0].get(0, 0), Some(&Tile::Ash));
        assert_eq!(map.patterns[0].get(1, 0), Some(&Tile::Rock));
        assert_eq!(map.patterns[0].get(2, 2), Some(&Tile::Ash));
        assert_eq!(map.patterns[0].get(3, 2), Some(&Tile::Rock));
    }

    #[test]
    fn find_reflection_across_vertical() {
        let input = "#.##..##.
..#.##.#.";

        let mut map: Map = input.parse().unwrap();

        assert_eq!(find_reflection(map.patterns.pop().unwrap()), 5);
    }

    #[test]
    fn find_reflection_across_horizontal() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let mut map: Map = input.parse().unwrap();

        assert_eq!(find_reflection(map.patterns.pop().unwrap()), 400);
    }

    #[test]
    fn find_reflection_across_horizontal_with_smudge() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

        let mut map: Map = input.parse().unwrap();

        assert_eq!(
            find_reflection_with_smudge(map.patterns.pop().unwrap()),
            100
        );
    }
}
