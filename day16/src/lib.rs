mod grid;

use std::collections::{HashSet, VecDeque};

use grid::Grid;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_in(self, direction: Direction) -> Option<Self> {
        let position = match direction {
            Direction::North => Self::new(self.x, self.y.checked_sub(1)?),
            Direction::South => Self::new(self.x, self.y.checked_add(1)?),
            Direction::West => Self::new(self.x.checked_sub(1)?, self.y),
            Direction::East => Self::new(self.x.checked_add(1)?, self.y),
        };

        Some(position)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    RightSlopedMirror,
    LeftSlopedMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = &'static str;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        let tile = match char {
            '.' => Tile::Empty,
            '/' => Tile::RightSlopedMirror,
            '\\' => Tile::LeftSlopedMirror,
            '|' => Tile::VerticalSplitter,
            '-' => Tile::HorizontalSplitter,
            _ => return Err("Invalid character encountered"),
        };

        Ok(tile)
    }
}

impl From<Tile> for char {
    fn from(tile: Tile) -> Self {
        match tile {
            Tile::Empty => '.',
            Tile::RightSlopedMirror => '/',
            Tile::LeftSlopedMirror => '\\',
            Tile::VerticalSplitter => '|',
            Tile::HorizontalSplitter => '-',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn interact(self, tile: Tile) -> (Self, Option<Self>) {
        use Direction::*;
        use Tile::*;
        match (tile, self) {
            (Empty, _) => (self, None),
            (RightSlopedMirror, North) => (East, None),
            (RightSlopedMirror, South) => (West, None),
            (RightSlopedMirror, West) => (South, None),
            (RightSlopedMirror, East) => (North, None),
            (LeftSlopedMirror, North) => (West, None),
            (LeftSlopedMirror, South) => (East, None),
            (LeftSlopedMirror, West) => (North, None),
            (LeftSlopedMirror, East) => (South, None),
            (VerticalSplitter, North | South) => (self, None),
            (VerticalSplitter, West | East) => (North, Some(South)),
            (HorizontalSplitter, North | South) => (West, Some(East)),
            (HorizontalSplitter, West | East) => (self, None),
        }
    }
}

fn parse_input(input: &str) -> Grid<Tile> {
    let lines = input.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len();
    let tiles: Vec<_> = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(Tile::try_from)
        .collect::<Result<_, _>>()
        .expect("Failed to parse puzzle input");

    Grid::new(width, height, tiles)
}

fn count_energized(grid: &Grid<Tile>, start_pos: Position, start_dir: Direction) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start_pos, start_dir));

    let mut visited = HashSet::new();
    let mut energized = HashSet::new();
    while let Some((mut position, mut direction)) = queue.pop_front() {
        while let Some(&tile) = grid.get(position.x, position.y) {
            if !visited.insert((position, direction)) {
                break;
            }

            energized.insert(position);

            let (dir, new_dir) = direction.interact(tile);
            if let Some(new_dir) = new_dir {
                if let Some(new_pos) = position.move_in(new_dir) {
                    queue.push_back((new_pos, new_dir));
                }
            }

            direction = dir;
            if let Some(pos) = position.move_in(dir) {
                position = pos;
            } else {
                break;
            }
        }
    }

    energized.len()
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = parse_input(input);

    let start_pos = Position::new(0, 0);
    let start_dir = Direction::East;

    // for y in 0..grid.height {
    //     for x in 0..grid.width {
    //         let pos = Position::new(x, y);
    //         let char = if energized.contains(&pos) { '#' } else { '.' };
    //         print!("{}", char)
    //     }
    //     println!()
    // }

    count_energized(&grid, start_pos, start_dir)
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = parse_input(input);

    (0..grid.width)
        .map(|x| (Position::new(x, 0), Direction::South))
        .chain((0..grid.height).map(|y| (Position::new(0, y), Direction::East)))
        .chain((0..grid.width).map(|x| (Position::new(x, grid.height - 1), Direction::North)))
        .chain((0..grid.height).map(|y| (Position::new(grid.width - 1, y), Direction::West)))
        .map(|(pos, dir)| count_energized(&grid, pos, dir))
        .max()
        .unwrap()
}
