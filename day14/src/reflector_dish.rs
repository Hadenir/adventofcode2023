use std::str::FromStr;

use itertools::Itertools;

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    RoundedRock,
    CubeRock,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        let tile = match char {
            '.' => Tile::Empty,
            'O' => Tile::RoundedRock,
            '#' => Tile::CubeRock,
            _ => return Err("Invalid character encountered"),
        };

        Ok(tile)
    }
}

pub struct ReflectorDish {
    pub grid: Grid<Tile>,
}

impl FromStr for ReflectorDish {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().collect_vec();
        let height = lines.len();
        let width = lines[0].len();
        let tiles = lines
            .into_iter()
            .flat_map(|line| line.chars())
            .map(|char| char.try_into().unwrap());

        Ok(Self {
            grid: Grid::new(width, height, tiles),
        })
    }
}
