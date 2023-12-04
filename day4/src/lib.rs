mod card;
mod parser;

use std::collections::HashMap;

use parser::parse_input;

pub fn solve_part_1(input: &str) -> u32 {
    let cards = parse_input(input);

    cards.into_iter().map(|card| card.score()).sum()
}

pub fn solve_part_2(input: &str) -> u32 {
    let cards = parse_input(input);

    // Maps card id to number of cards that card directly wins.
    let base_won_cards = cards
        .into_iter()
        .map(|card| (card.id, card.won_cards()));

    let mut won_cards = HashMap::with_capacity(base_won_cards.len());
    for (card_id, base_card_score) in base_won_cards.rev() {
        let card_score: u32 = (1..=base_card_score).map(|i| won_cards[&(card_id + i as usize)]).sum();
        won_cards.insert(card_id, base_card_score + card_score);
    }

    won_cards.len() as u32 + won_cards.into_values().sum::<u32>()
}
