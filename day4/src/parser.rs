use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    Finish, IResult,
};

use crate::card::ScratchCard;

pub(crate) fn parse_input(input: &str) -> Vec<ScratchCard> {
    let (_, cards) = card_list(input)
        .finish()
        .expect("Failed to parse puzzle input");

    cards
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn number_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, number)(input)
}

fn card_id(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn scratch_card(input: &str) -> IResult<&str, ScratchCard> {
    map(
        tuple((
            delimited(
                tuple((tag("Card"), space1)),
                card_id,
                tuple((char(':'), space1)),
            ),
            terminated(number_list, tuple((space1, char('|'), space1))),
            number_list,
        )),
        |(id, winning_numbers, numbers)| ScratchCard::new(id, winning_numbers, numbers),
    )(input)
}

fn card_list(input: &str) -> IResult<&str, Vec<ScratchCard>> {
    separated_list1(line_ending, scratch_card)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_list() {
        let input = "12 34 5 123 0";

        let (rem, numbers) = number_list(input).unwrap();

        assert_eq!(numbers, vec![12, 34, 5, 123, 0]);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_scratch_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

        let (rem, card) = scratch_card(input).unwrap();

        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
        assert!(rem.is_empty());
    }

    #[test]
    fn test_parse_card_list() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";

        let (rem, cards) = card_list(input).unwrap();

        assert_eq!(cards.len(), 2);
        assert_eq!(cards[0].id, 2);
        assert_eq!(cards[0].winning_numbers, vec![13, 32, 20, 16, 61]);
        assert_eq!(cards[0].numbers, vec![61, 30, 68, 82, 17, 32, 24, 19]);
        assert_eq!(cards[1].id, 3);
        assert_eq!(cards[1].winning_numbers, vec![1, 21, 53, 59, 44]);
        assert_eq!(cards[1].numbers, vec![69, 82, 63, 72, 16, 21, 14, 1]);
        assert!(rem.is_empty());
    }
}
