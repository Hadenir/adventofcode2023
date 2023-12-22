use std::collections::{HashMap, HashSet};

use grid::Grid;
use itertools::Itertools;

mod grid;

fn parse_input(input: &str) -> Grid<u8> {
    let lines = input.lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len();
    let cells = lines.into_iter().flat_map(|line| line.chars()).map(|char| {
        char.to_digit(10)
            .expect("Map does not contain non-digit characters") as u8
    });

    Grid::new(width, height, cells)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn manhattan_distance(p0: Point, p1: Point) -> usize {
    p0.x.abs_diff(p1.x) + p0.y.abs_diff(p1.y)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    point: Point,
    direction: Direction,
    moves_in_dir: u8,
}

impl Node {
    fn new(point: Point, direction: Direction, moves_in_dir: u8) -> Self {
        Self {
            point,
            direction,
            moves_in_dir,
        }
    }
}

fn _reconstruct_path(came_from: HashMap<Node, Node>, node: Node) -> Vec<Node> {
    let mut nodes = vec![node];

    let mut current_node = &node;
    while let Some(node) = came_from.get(current_node) {
        nodes.push(*node);
        current_node = node;
    }

    nodes.reverse();
    nodes
}

pub fn solve_part_1(input: &str) -> u64 {
    let grid = parse_input(input);

    let end_point = Point::new(grid.width - 1, grid.height - 1);
    let point_a = Point::new(1, 0);
    let point_b = Point::new(0, 1);

    let node_a = Node::new(point_a, Direction::East, 1);
    let node_b = Node::new(point_b, Direction::South, 1);

    let mut g_score = HashMap::<Node, u64>::new();
    g_score.insert(node_a, grid.get(point_a.x, point_a.y) as u64);
    g_score.insert(node_b, grid.get(point_b.x, point_b.y) as u64);

    let mut f_score = HashMap::<Node, u64>::new();
    f_score.insert(
        node_a,
        g_score[&node_a] + manhattan_distance(point_a, end_point) as u64,
    );
    f_score.insert(
        node_b,
        g_score[&node_b] + manhattan_distance(point_b, end_point) as u64,
    );

    let mut open_nodes = HashSet::<Node>::new();
    open_nodes.insert(node_a);
    open_nodes.insert(node_b);

    let mut came_from = HashMap::<Node, Node>::new();

    while let Some(node) = open_nodes.iter().min_by_key(|node| f_score[node]).cloned() {
        open_nodes.remove(&node);

        if node.point == end_point {
            // let path = reconstruct_path(came_from, node);
            // for y in 0..grid.height {
            //     for x in 0..grid.width {
            //         let point = Point::new(x, y);
            //         print!(
            //             "{}",
            //             if let Some(node) = path.iter().find(|node| node.point == point) {
            //                 match node.direction {
            //                     Direction::North => '^',
            //                     Direction::South => 'v',
            //                     Direction::West => '<',
            //                     Direction::East => '>',
            //                 }
            //             } else {
            //                 char::from_digit(grid.get(point.x, point.y) as u32, 12).unwrap()
            //             }
            //         );
            //     }
            //     println!();
            // }
            return g_score[&node];
        }

        let mut f = |new_point: Point, direction: Direction| {
            if node.direction != direction || node.moves_in_dir < 3 {
                let moves_in_dir = if node.direction == direction {
                    node.moves_in_dir + 1
                } else {
                    1
                };
                let new_node = Node::new(new_point, direction, moves_in_dir);

                let heat_loss = g_score[&node] + grid.get(new_point.x, new_point.y) as u64;
                if heat_loss < g_score.get(&new_node).cloned().unwrap_or(u64::MAX) {
                    g_score.insert(new_node, heat_loss);
                    f_score.insert(
                        new_node,
                        heat_loss + manhattan_distance(new_point, end_point) as u64,
                    );

                    came_from.insert(new_node, node);

                    open_nodes.insert(new_node);
                    // let new_node =
                    //     Node::new(new_point, f_score[&new_point], direction, moves_in_dir);
                    // open_nodes.retain(|node| node != &new_node);
                    // open_nodes.push(new_node);
                }
            }
        };

        if node.point.y > 0 && node.direction != Direction::South {
            f(Point::new(node.point.x, node.point.y - 1), Direction::North);
        }
        if node.point.y < grid.height - 1 && node.direction != Direction::North {
            f(Point::new(node.point.x, node.point.y + 1), Direction::South);
        }
        if node.point.x > 0 && node.direction != Direction::East {
            f(Point::new(node.point.x - 1, node.point.y), Direction::West);
        }
        if node.point.x < grid.width - 1 && node.direction != Direction::West {
            f(Point::new(node.point.x + 1, node.point.y), Direction::East);
        }
    }

    panic!("no path was found")
}

pub fn solve_part_2(input: &str) -> u64 {
    let grid = parse_input(input);

    let end_point = Point::new(grid.width - 1, grid.height - 1);
    let point_a = Point::new(1, 0);
    let point_b = Point::new(0, 1);

    let node_a = Node::new(point_a, Direction::East, 1);
    let node_b = Node::new(point_b, Direction::South, 1);

    let mut g_score = HashMap::<Node, u64>::new();
    g_score.insert(node_a, grid.get(point_a.x, point_a.y) as u64);
    g_score.insert(node_b, grid.get(point_b.x, point_b.y) as u64);

    let mut f_score = HashMap::<Node, u64>::new();
    f_score.insert(
        node_a,
        g_score[&node_a] + manhattan_distance(point_a, end_point) as u64,
    );
    f_score.insert(
        node_b,
        g_score[&node_b] + manhattan_distance(point_b, end_point) as u64,
    );

    let mut open_nodes = HashSet::<Node>::new();
    open_nodes.insert(node_a);
    open_nodes.insert(node_b);

    let mut came_from = HashMap::<Node, Node>::new();

    while let Some(node) = open_nodes.iter().min_by_key(|node| f_score[node]).cloned() {
        open_nodes.remove(&node);

        if node.point == end_point && node.moves_in_dir >= 4 {
            // let path = reconstruct_path(came_from, node);
            // for y in 0..grid.height {
            //     for x in 0..grid.width {
            //         let point = Point::new(x, y);
            //         print!(
            //             "{}",
            //             if let Some(node) = path.iter().find(|node| node.point == point) {
            //                 match node.direction {
            //                     Direction::North => '^',
            //                     Direction::South => 'v',
            //                     Direction::West => '<',
            //                     Direction::East => '>',
            //                 }
            //             } else {
            //                 char::from_digit(grid.get(point.x, point.y) as u32, 12).unwrap()
            //             }
            //         );
            //     }
            //     println!();
            // }
            return g_score[&node];
        }

        let mut f = |new_point: Point, direction: Direction| {
            if node.direction != direction || node.moves_in_dir < 10 {
                let moves_in_dir = if node.direction == direction {
                    node.moves_in_dir + 1
                } else {
                    1
                };
                let new_node = Node::new(new_point, direction, moves_in_dir);

                let heat_loss = g_score[&node] + grid.get(new_point.x, new_point.y) as u64;
                if heat_loss < g_score.get(&new_node).cloned().unwrap_or(u64::MAX) {
                    g_score.insert(new_node, heat_loss);
                    f_score.insert(
                        new_node,
                        heat_loss + manhattan_distance(new_point, end_point) as u64,
                    );

                    came_from.insert(new_node, node);

                    open_nodes.insert(new_node);
                    // let new_node =
                    //     Node::new(new_point, f_score[&new_point], direction, moves_in_dir);
                    // open_nodes.retain(|node| node != &new_node);
                    // open_nodes.push(new_node);
                }
            }
        };

        if node.moves_in_dir < 4 {
            match node.direction {
                Direction::North if node.point.y > 0 => f(Point::new(node.point.x, node.point.y - 1), node.direction),
                Direction::South if node.point.y < grid.height - 1 => f(Point::new(node.point.x, node.point.y + 1), node.direction),
                Direction::West if node.point.x > 0 => f(Point::new(node.point.x - 1, node.point.y), node.direction),
                Direction::East if node.point.x < grid.width - 1 => f(Point::new(node.point.x + 1, node.point.y), node.direction),
                _ => (),
            }
            continue;
        }

        if node.point.y > 0 && node.direction != Direction::South {
            f(Point::new(node.point.x, node.point.y - 1), Direction::North);
        }
        if node.point.y < grid.height - 1 && node.direction != Direction::North {
            f(Point::new(node.point.x, node.point.y + 1), Direction::South);
        }
        if node.point.x > 0 && node.direction != Direction::East {
            f(Point::new(node.point.x - 1, node.point.y), Direction::West);
        }
        if node.point.x < grid.width - 1 && node.direction != Direction::West {
            f(Point::new(node.point.x + 1, node.point.y), Direction::East);
        }
    }

    panic!("no path was found")
}
