mod camel_cards;
mod parser;

use camel_cards::Card;
use itertools::Itertools;
use parser::parse_input;

pub fn solve_part_1(input: &str) -> u64 {
    let games = parse_input(input);

    games
        .into_iter()
        .sorted_by(|a, b| a.hand.cmp(&b.hand))
        .enumerate()
        .map(|(rank, game)| (rank + 1) as u64 * game.bid)
        .sum()
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut games = parse_input(input);

    // Replace jacks with jokers.
    for game in &mut games {
        for card in &mut game.hand.cards {
            if card == &Card::Jack {
                *card = Card::Joker;
            }
        }
    }

    games
        .into_iter()
        .sorted_by(|a, b| a.hand.cmp(&b.hand))
        .enumerate()
        .map(|(rank, game)| (rank + 1) as u64 * game.bid)
        .sum()
}
