mod parser;

use hashbrown::HashMap;

use itertools::Itertools;
use parser::parse_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Left,
    Right,
}

impl From<char> for Move {
    fn from(char: char) -> Self {
        match char {
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("Invalid move char"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<'a>(&'a str);

impl<'a> Node<'a> {
    pub fn new(tag: &'a str) -> Self {
        Self(tag)
    }
}

pub struct Map<'a> {
    pub moves: Vec<Move>,
    pub nodes: HashMap<Node<'a>, (Node<'a>, Node<'a>)>,
}

impl<'a> Map<'a> {
    pub fn new(moves: Vec<Move>, nodes: Vec<(Node<'a>, (Node<'a>, Node<'a>))>) -> Self {
        Self {
            moves,
            nodes: nodes.into_iter().collect(),
        }
    }

    pub fn get(&self, node: &Node<'a>, r#move: Move) -> &Node<'a> {
        let (left, right) = &self.nodes[node];
        match r#move {
            Move::Left => left,
            Move::Right => right,
        }
    }
}

pub fn solve_part_1(input: &str) -> u64 {
    let map = parse_input(input);

    let start_node = Node::new("AAA");
    let end_node = Node::new("ZZZ");

    let mut current_node = &start_node;
    let mut num_moves = 0;
    for r#move in map.moves.iter().cycle() {
        let next_node = map.get(current_node, *r#move);
        current_node = next_node;
        num_moves += 1;

        if current_node == &end_node {
            break;
        }
    }

    num_moves
}

fn least_common_multiple(numbers: &[u64]) -> Option<u64> {
    match numbers.len() {
        0 => return None,
        1 => return Some(numbers[0]),
        _ => (),
    }

    let a = numbers[0];
    let b = least_common_multiple(&numbers[1..])?;
    Some(a * b / greatest_common_divisor(a, b))
}

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    match b {
        0 => a,
        _ => greatest_common_divisor(b, a % b),
    }
}

pub fn solve_part_2(input: &str) -> u64 {
    let map = parse_input(input);

    let num_moves = map
        .nodes
        .keys()
        .filter(|node| node.0.ends_with('A'))
        .map(|node| {
            let mut current_node = node;
            let mut num_moves = 0u64;
            for r#move in map.moves.iter().cycle() {
                let next_node = map.get(current_node, *r#move);
                current_node = next_node;
                num_moves += 1;

                if current_node.0.ends_with('Z'){
                    break;
                }
            }

            num_moves
        })
        .collect_vec();

    least_common_multiple(&num_moves).unwrap()
}

// This was too slow :(
// pub fn solve_part_2(input: &str) -> usize {
//     let map = parse_input(input);

//     let start_nodes = map
//         .nodes
//         .keys()
//         .filter(|node| node.0.ends_with('A'))
//         .collect_vec();

//     let mut current_nodes = start_nodes;
//     let mut num_moves = 0;
//     for r#move in map.moves.iter().cycle() {
//         for current_node in &mut current_nodes {
//             let next_node = map.get(current_node, *r#move);
//             *current_node = next_node;
//         }
//         num_moves += 1;

//         if current_nodes.iter().all(|node| node.0.ends_with('Z')) {
//             break;
//         }
//     }

//     num_moves
// }
