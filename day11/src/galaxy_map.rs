use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn distance(self, other: Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub struct GalaxyMap {
    pub galaxies: Vec<Position>,
    pub empty_columns: Vec<usize>,
    pub empty_rows: Vec<usize>,
}

impl FromStr for GalaxyMap {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut galaxies = Vec::new();
        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.char_indices() {
                match char {
                    '#' => galaxies.push(Position::new(x, y)),
                    '.' => continue,
                    _ => return Err("Invalid character encountered in map"),
                }
            }
        }

        let (width, height) = galaxies.iter().fold((0, 0), |(acc_w, acc_h), pos| {
            (acc_w.max(pos.x), acc_h.max(pos.y))
        });

        let empty_columns = (0..=width)
            .filter(|x| galaxies.iter().all(|pos| &pos.x != x))
            .collect();
        let empty_rows = (0..=height)
            .filter(|y| galaxies.iter().all(|pos| &pos.y != y))
            .collect();

        Ok(Self {
            galaxies,
            empty_columns,
            empty_rows,
        })
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn parse_galaxy_map() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let map: GalaxyMap = input.parse().unwrap();

        assert_eq!(
            map.galaxies,
            vec![
                Position::new(3, 0),
                Position::new(7, 1),
                Position::new(0, 2),
                Position::new(6, 4),
                Position::new(1, 5),
                Position::new(9, 6),
                Position::new(7, 8),
                Position::new(0, 9),
                Position::new(4, 9),
            ]
        );
        assert_eq!(map.empty_columns.into_iter().collect_vec(), vec![2, 5, 8],);
        assert_eq!(map.empty_rows.into_iter().collect_vec(), vec![3, 7]);
    }
}
