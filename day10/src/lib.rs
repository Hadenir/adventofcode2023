use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pipe {
    Empty,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    fn is_connected_to(&self, direction: Direction) -> bool {
        match direction {
            Direction::North => matches!(self, Pipe::NorthEast | Pipe::NorthWest | Pipe::Vertical),
            Direction::East => matches!(self, Pipe::SouthEast | Pipe::NorthWest | Pipe::Horizontal),
            Direction::South => matches!(self, Pipe::SouthEast | Pipe::SouthWest | Pipe::Vertical),
            Direction::West => matches!(self, Pipe::NorthWest | Pipe::SouthWest | Pipe::Horizontal),
        }
    }
}

impl TryFrom<char> for Pipe {
    type Error = &'static str;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        use Pipe::*;

        match char {
            '|' => Ok(Vertical),
            '-' => Ok(Horizontal),
            'L' => Ok(NorthEast),
            'J' => Ok(NorthWest),
            '7' => Ok(SouthWest),
            'F' => Ok(SouthEast),
            '.' => Ok(Empty),
            a => {
                dbg!(a);
                Err("Invalid character in the map")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn offset(&self, offset_x: isize, offset_y: isize) -> Self {
        Self {
            x: if offset_x < 0 {
                self.x - offset_x.unsigned_abs()
            } else {
                self.x + offset_x as usize
            },
            y: if offset_y < 0 {
                self.y - offset_y.unsigned_abs()
            } else {
                self.y + offset_y as usize
            },
        }
    }
}

struct Grid<T> {
    width: usize,
    height: usize,
    tiles: Vec<T>,
}

impl<T: Copy> Grid<T> {
    fn get(&self, position: Position) -> T {
        self.tiles[position.x + position.y * self.width]
    }

    fn get_mut(&mut self, position: Position) -> &mut T {
        &mut self.tiles[position.x + position.y * self.width]
    }
}

struct Map {
    start: Position,
    grid: Grid<Pipe>,
}

impl Map {
    fn get_start(&self) -> Pipe {
        self.grid.get(self.start)
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines().collect_vec();
        let width = lines[0].len();
        let height = lines.len();

        let mut tiles = Vec::with_capacity(width * height);

        let mut start_idx = 0;
        for (idx, char) in lines.into_iter().flat_map(str::chars).enumerate() {
            if char == 'S' {
                start_idx = idx;
                tiles.push(Pipe::Empty);
            } else {
                tiles.push(char.try_into()?);
            }
        }

        let start = Position {
            x: start_idx % width,
            y: start_idx / width,
        };

        let north_pipe = (start.y > 0).then(|| tiles[start_idx - width]);
        let east_pipe = (start.x < width - 1).then(|| tiles[start_idx + 1]);
        let south_pipe = (start.y < width - 1).then(|| tiles[start_idx + width]);
        let west_pipe = (start.x > 0).then(|| tiles[start_idx - 1]);

        use Pipe::*;
        let start_pipe = match (north_pipe, east_pipe, south_pipe, west_pipe) {
            (_, Some(east_pipe), _, Some(west_pipe))
                if east_pipe.is_connected_to(Direction::West)
                    && west_pipe.is_connected_to(Direction::East) =>
            {
                Horizontal
            }
            (Some(north_pipe), _, Some(south_pipe), _)
                if north_pipe.is_connected_to(Direction::South)
                    && south_pipe.is_connected_to(Direction::North) =>
            {
                Vertical
            }
            (Some(north_pipe), Some(east_pipe), _, _)
                if north_pipe.is_connected_to(Direction::South)
                    && east_pipe.is_connected_to(Direction::West) =>
            {
                NorthEast
            }
            (Some(north_pipe), _, _, Some(west_pipe))
                if north_pipe.is_connected_to(Direction::South)
                    && west_pipe.is_connected_to(Direction::East) =>
            {
                NorthWest
            }
            (_, _, Some(south_pipe), Some(west_pipe))
                if south_pipe.is_connected_to(Direction::North)
                    && west_pipe.is_connected_to(Direction::East) =>
            {
                SouthWest
            }
            (_, Some(east_pipe), Some(south_pipe), _)
                if east_pipe.is_connected_to(Direction::West)
                    && south_pipe.is_connected_to(Direction::North) =>
            {
                SouthEast
            }
            _ => return Err("Invalid start pipe"),
        };

        tiles[start_idx] = start_pipe;

        Ok(Self {
            start,
            grid: Grid {
                width,
                height,
                tiles,
            },
        })
    }
}

fn move_in_direction(mut position: Position, direction: Direction) -> Position {
    match direction {
        Direction::North => position.y -= 1,
        Direction::East => position.x += 1,
        Direction::South => position.y += 1,
        Direction::West => position.x -= 1,
    }

    position
}

fn outgoing_direction(pipe: Pipe, incoming_direction: Direction) -> Direction {
    match (pipe, incoming_direction) {
        (Pipe::Vertical, Direction::North) => Direction::North,
        (Pipe::Vertical, Direction::South) => Direction::South,
        (Pipe::Horizontal, Direction::West) => Direction::West,
        (Pipe::NorthEast, Direction::South) => Direction::East,
        (Pipe::NorthEast, Direction::West) => Direction::North,
        (Pipe::NorthWest, Direction::South) => Direction::West,
        (Pipe::NorthWest, Direction::East) => Direction::North,
        (Pipe::SouthWest, Direction::North) => Direction::West,
        (Pipe::SouthWest, Direction::East) => Direction::South,
        (Pipe::SouthEast, Direction::North) => Direction::East,
        (Pipe::SouthEast, Direction::West) => Direction::South,
        (Pipe::Horizontal, Direction::East) => Direction::East,
        a => {
            dbg!(a);
            unreachable!()
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    let start_pipe = map.get_start();

    // `A`` path goes in "clockwise" direction along the pipe.
    let mut position_a = map.start;
    let mut direction_a = match start_pipe {
        Pipe::Vertical => Direction::North,
        Pipe::Horizontal => Direction::East,
        Pipe::NorthEast => Direction::North,
        Pipe::NorthWest => Direction::North,
        Pipe::SouthWest => Direction::South,
        Pipe::SouthEast => Direction::South,
        Pipe::Empty => unreachable!(),
    };

    // `B`` path goes in "anti-clockwise" direction along the pipe.
    let mut position_b = map.start;
    let mut direction_b = match start_pipe {
        Pipe::Vertical => Direction::South,
        Pipe::Horizontal => Direction::West,
        Pipe::NorthEast => Direction::East,
        Pipe::NorthWest => Direction::West,
        Pipe::SouthWest => Direction::West,
        Pipe::SouthEast => Direction::East,
        Pipe::Empty => unreachable!(),
    };

    let mut length = 0;
    loop {
        length += 1;

        // Take one step along `A` and `B` paths.
        position_a = move_in_direction(position_a, direction_a);
        position_b = move_in_direction(position_b, direction_b);

        // If `A` and `B` meet, we traversed the whole loop.
        if position_a == position_b {
            break;
        }

        // Pick next direction based on incoming direction and shape of the pipe.
        direction_a = outgoing_direction(map.grid.get(position_a), direction_a);
        direction_b = outgoing_direction(map.grid.get(position_b), direction_b);
    }

    length
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Red,
    Green,
    Black,
}

impl Display for Grid<Option<Color>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let char = match self.get(Position { x, y }) {
                    None => ' ',
                    Some(Color::Green) => 'O',
                    Some(Color::Red) => '.',
                    Some(Color::Black) => '+',
                };
                write!(f, "{}", char)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn neighbors_colors(
    pipe: Pipe,
    incoming_direction: Direction,
) -> (Option<Color>, Option<Color>, Option<Color>, Option<Color>) {
    match (pipe, incoming_direction) {
        (Pipe::Vertical, Direction::North) => (None, Some(Color::Green), None, Some(Color::Red)),
        (Pipe::Vertical, Direction::South) => (None, Some(Color::Red), None, Some(Color::Green)),
        (Pipe::Horizontal, Direction::East) => (Some(Color::Red), None, Some(Color::Green), None),
        (Pipe::Horizontal, Direction::West) => (Some(Color::Green), None, Some(Color::Red), None),
        (Pipe::NorthEast, Direction::South) => (None, None, Some(Color::Green), Some(Color::Green)),
        (Pipe::NorthEast, Direction::West) => (None, None, Some(Color::Red), Some(Color::Red)),
        (Pipe::NorthWest, Direction::South) => (None, Some(Color::Red), Some(Color::Red), None),
        (Pipe::NorthWest, Direction::East) => (None, Some(Color::Green), Some(Color::Green), None),
        (Pipe::SouthWest, Direction::North) => (Some(Color::Green), Some(Color::Green), None, None),
        (Pipe::SouthWest, Direction::East) => (Some(Color::Red), Some(Color::Red), None, None),
        (Pipe::SouthEast, Direction::North) => (Some(Color::Red), None, None, Some(Color::Red)),
        (Pipe::SouthEast, Direction::West) => (Some(Color::Green), None, None, Some(Color::Green)),
        a => {
            dbg!(a);
            unreachable!()
        }
    }
}

pub fn solve_part_2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();

    let start_pipe = map.get_start();

    let mut position = map.start;
    let mut direction = match start_pipe {
        Pipe::Vertical => Direction::North,
        Pipe::Horizontal => Direction::East,
        Pipe::NorthEast => Direction::West,
        Pipe::NorthWest => Direction::South,
        Pipe::SouthWest => Direction::East,
        Pipe::SouthEast => Direction::North,
        Pipe::Empty => unreachable!(),
    };

    let mut colors: Grid<Option<Color>> = Grid {
        width: map.grid.width,
        height: map.grid.height,
        tiles: vec![None; map.grid.width * map.grid.height],
    };

    loop {
        *colors.get_mut(position) = Some(Color::Black);

        let (north_color, east_color, south_color, west_color) =
            neighbors_colors(map.grid.get(position), direction);

        (position.y > 0 && colors.get(position.offset(0, -1)).is_none()).then(|| {
            north_color.map(|color| {
                let current_color = colors.get_mut(position.offset(0, -1));
                if let Some(current_color) = current_color {
                    assert_eq!(*current_color, color);
                } else {
                    *current_color = Some(color);
                }
            })
        });
        (position.x < colors.width - 1 && colors.get(position.offset(1, 0)).is_none()).then(
            || {
                east_color.map(|color| {
                    let current_color = colors.get_mut(position.offset(1, 0));
                    if let Some(current_color) = current_color {
                        assert_eq!(*current_color, color);
                    } else {
                        *current_color = Some(color);
                    }
                })
            },
        );
        (position.y < colors.height - 1 && colors.get(position.offset(0, 1)).is_none())
            .then(|| {
                south_color.map(|color| {
                    let current_color = colors.get_mut(position.offset(0, 1));
                    if let Some(current_color) = current_color {
                        assert_eq!(*current_color, color);
                    } else {
                        *current_color = Some(color);
                    }
                })
            });
        (position.x > 0 && colors.get(position.offset(-1, 0)).is_none()).then(|| {
            west_color.map(|color| {
                let current_color = colors.get_mut(position.offset(-1, 0));
                if let Some(current_color) = current_color {
                    assert_eq!(*current_color, color);
                } else {
                    *current_color = Some(color);
                }
            })
        });

        direction = outgoing_direction(map.grid.get(position), direction);
        position = move_in_direction(position, direction);
        if position == map.start {
            break;
        }
    }

    let mut candidates = VecDeque::new();
    candidates.extend(
        (0..colors.height)
            .cartesian_product(0..colors.width)
            .filter_map(|(y, x)| {
                let position = Position { x, y };
                (colors.get(position) == Some(Color::Green)).then_some(position)
            }),
    );

    let mut visited = HashSet::<Position>::new();
    while let Some(position) = candidates.pop_front() {
        if colors.get(position) != Some(Color::Black) {
            if colors.get(position) == Some(Color::Red) {
                panic!("PANIC!");
            }

            visited.insert(position);

            *colors.get_mut(position) = Some(Color::Green);

            if let Some(position) = (position.y > 0)
                .then(|| position.offset(0, -1))
                .filter(|position| !visited.contains(position))
            {
                candidates.push_back(position)
            }
            if let Some(position) = (position.x < colors.width - 1)
                .then(|| position.offset(1, 0))
                .filter(|position| !visited.contains(position))
            {
                candidates.push_back(position)
            }
            if let Some(position) = (position.y < colors.height - 1)
                .then(|| position.offset(0, 1))
                .filter(|position| !visited.contains(position))
            {
                candidates.push_back(position)
            }
            if let Some(position) = (position.x > 0)
                .then(|| position.offset(-1, 0))
                .filter(|position| !visited.contains(position))
            {
                candidates.push_back(position)
            }
        }
    }

    println!("{}", colors);

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_replace_start_pipe_1() {
        let input = ".....
.F-7.
.|.|.
.S-J.
.....";

        let map: Map = input.parse().unwrap();

        assert_eq!(map.grid.get(map.start), Pipe::NorthEast);
    }

    #[test]
    fn correctly_replace_start_pipe_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let map: Map = input.parse().unwrap();

        assert_eq!(map.grid.get(map.start), Pipe::SouthEast);
    }

    #[test]
    fn correctly_replace_start_pipe_3() {
        let input = ".F7.
.S|.
.LJ.";

        let map: Map = input.parse().unwrap();

        assert_eq!(map.grid.get(map.start), Pipe::Vertical);
    }
}
