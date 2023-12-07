use nom::{
    character::complete::{digit1, line_ending, one_of, space1},
    combinator::{map, map_res},
    multi::{count, separated_list1},
    sequence::separated_pair,
    Finish, IResult,
};

use crate::camel_cards::{Card, Game, Hand};

pub(crate) fn parse_input(input: &str) -> Vec<Game> {
    let (_, games) = game_list(input)
        .finish()
        .expect("Failed to parse puzzle input");

    games
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    map_res(one_of("AKQJT98765432"), Card::try_from)(input)
}

fn hand(input: &str) -> IResult<&str, Hand> {
    map(count(card, 5), |cards| Hand::new(cards.try_into().unwrap()))(input)
}

fn game(input: &str) -> IResult<&str, Game> {
    map(separated_pair(hand, space1, number), |(hand, bid)| {
        Game::new(hand, bid)
    })(input)
}

fn game_list(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(line_ending, game)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_card() {
        let input = "Q";

        let (rem, card) = card(input).unwrap();

        assert_eq!(card, Card::Queen);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_hand() {
        let input = "32T3K";

        let (rem, hand) = hand(input).unwrap();

        assert_eq!(
            hand.cards,
            [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]
        );
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_game() {
        let input = "QQQJA 483";

        let (rem, game) = game(input).unwrap();

        assert_eq!(
            game.hand.cards,
            [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
        );
        assert_eq!(game.bid, 483);
        assert!(rem.is_empty());
    }

    #[test]
    fn parse_game_list() {
        let input = "KTJJT 220
QQQJA 483";

        let (rem, games) = game_list(input).unwrap();

        assert_eq!(games.len(), 2);
        assert_eq!(
            games[0].hand.cards,
            [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]
        );
        assert_eq!(games[0].bid, 220);
        assert_eq!(
            games[1].hand.cards,
            [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]
        );
        assert_eq!(games[1].bid, 483);
        assert!(rem.is_empty());
    }
}
