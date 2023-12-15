use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    Finish, IResult,
};

use crate::*;

pub(crate) fn parse_input(input: &str) -> Vec<Instruction> {
    let (_, instructions) = instruction_list(input)
        .finish()
        .expect("Failed to parse puzzle input");

    instructions
}

fn label(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

fn focal_length(input: &str) -> IResult<&str, u8> {
    map_res(digit1, str::parse)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction<'_>> {
    alt((
        map(terminated(label, char('-')), |label| {
            Instruction::RemoveLens { label }
        }),
        map(
            separated_pair(label, char('='), focal_length),
            |(label, focal_length)| Instruction::InsertLens {
                label,
                focal_length,
            },
        ),
    ))(input)
}

fn instruction_list(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(char(','), instruction)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_remove_instruction() {
        let input = "abc-";

        let (rem, instruction) = instruction(input).unwrap();

        assert!(matches!(
            instruction,
            Instruction::RemoveLens { label: "abc" }
        ));
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_insert_instruction() {
        let input = "abc=2";

        let (rem, instruction) = instruction(input).unwrap();

        assert!(matches!(
            instruction,
            Instruction::InsertLens {
                label: "abc",
                focal_length: 2
            }
        ));
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_instruction_list() {
        let input = "abc=2,def-";

        let (rem, instructions) = instruction_list(input).unwrap();

        assert_eq!(instructions.len(), 2);
        assert!(matches!(
            instructions[0],
            Instruction::InsertLens {
                label: "abc",
                focal_length: 2
            }
        ));
        assert!(matches!(
            instructions[1],
            Instruction::RemoveLens { label: "def" }
        ));
        assert!(rem.is_empty());
    }
}
