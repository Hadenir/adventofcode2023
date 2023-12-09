use nom::{
    character::complete::{char, line_ending, one_of, space1, alphanumeric1},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    Finish, IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Map {
    let (_, map) = map_document(input)
        .finish()
        .expect("Failed to parse puzzle input");

    map
}

fn map_move(input: &str) -> IResult<&str, Move> {
    map(one_of("LR"), Move::from)(input)
}

fn map_moves(input: &str) -> IResult<&str, Vec<Move>> {
    many1(map_move)(input)
}

fn node(input: &str) -> IResult<&str, Node> {
    map(alphanumeric1, Node::new)(input)
}

fn map_entry(input: &str) -> IResult<&str, (Node, (Node, Node))> {
    separated_pair(
        node,
        tuple((space1, char('='), space1)),
        delimited(
            char('('),
            separated_pair(node, tuple((char(','), space1)), node),
            char(')'),
        ),
    )(input)
}

fn map_document(input: &str) -> IResult<&str, Map> {
    map(
        separated_pair(
            map_moves,
            many1(line_ending),
            separated_list1(line_ending, map_entry),
        ),
        |(moves, nodes)| Map::new(moves, nodes),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_map_moves() {
        let input = "RRL";

        let (rem, moves) = map_moves(input).unwrap();

        assert_eq!(moves, vec![Move::Right, Move::Right, Move::Left]);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_node() {
        let input = "ABC";

        let (rem, node) = node(input).unwrap();

        assert_eq!(node, Node::new("ABC"));
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_map_entry() {
        let input = "AAA = (BBB, CCC)";

        let (rem, entry) = map_entry(input).unwrap();

        assert_eq!(entry.0, Node::new("AAA"));
        assert_eq!(entry.1 .0, Node::new("BBB"));
        assert_eq!(entry.1 .1, Node::new("CCC"));
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_map() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (rem, map) = map_document(input).unwrap();

        assert_eq!(map.moves, vec![Move::Left, Move::Left, Move::Right]);
        assert_eq!(map.nodes.len(), 3);
        assert_eq!(
            map.nodes[&Node::new("AAA")],
            (Node::new("BBB"), Node::new("BBB"))
        );
        assert_eq!(
            map.nodes[&Node::new("BBB")],
            (Node::new("AAA"), Node::new("ZZZ"))
        );
        assert_eq!(
            map.nodes[&Node::new("ZZZ")],
            (Node::new("ZZZ"), Node::new("ZZZ"))
        );
        assert!(rem.is_empty());
    }
}
