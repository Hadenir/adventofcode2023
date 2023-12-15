use std::{
    fmt::Display,
    str::FromStr,
};

use itertools::Itertools;

use crate::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Tile::Empty => '.',
            Tile::RoundedRock => 'O',
            Tile::CubeRock => '#',
        };

        write!(f, "{}", char)
    }
}

#[derive(Hash)]
pub struct ReflectorDish {
    pub grid: Grid<Tile>,
}

impl ReflectorDish {
    pub fn north_load(&self) -> u64 {
        self.grid
            .iter_indices()
            .map(|(x, y)| match self.grid.get(x, y).unwrap() {
                Tile::RoundedRock => (self.grid.height - y) as u64,
                _ => 0,
            })
            .sum()
    }
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

impl Display for ReflectorDish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.grid.height {
            for x in 0..self.grid.width {
                write!(f, "{}", self.grid.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
