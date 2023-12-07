use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = &'static str;

    fn try_from(char: char) -> Result<Self, Self::Error> {
        use Card::*;

        let card = match char {
            'A' => Ace,
            'K' => King,
            'Q' => Queen,
            'J' => Jack,
            'T' => Ten,
            '9' => Nine,
            '8' => Eight,
            '7' => Seven,
            '6' => Six,
            '5' => Five,
            '4' => Four,
            '3' => Three,
            '2' => Two,
            _ => return Err("Invalid card encountered"),
        };

        Ok(card)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    pub cards: [Card; 5],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self { cards }
    }

    pub fn hand_type(&self) -> HandType {
        let card_counts = self
            .cards
            .iter()
            .counts()
            .into_iter()
            .sorted_by_key(|(_, count)| *count)
            .collect_vec();

        let contains_joker = self.cards.contains(&Card::Joker);

        match card_counts.len() {
            // 5
            1 => HandType::FiveOfAKind,
            // 4, 1
            2 if card_counts[1].1 == 4 => {
                if contains_joker {
                    // 4, 1J or 4J, 1
                    HandType::FiveOfAKind
                } else {
                    // 4, 1
                    HandType::FourOfAKind
                }
            }
            // 3, 2
            2 => {
                if contains_joker {
                    // 3J, 2 or 3, 2J
                    HandType::FiveOfAKind
                } else {
                    // 3, 2
                    HandType::FullHouse
                }
            }
            // 3, 1, 1
            3 if card_counts[2].1 == 3 => {
                if contains_joker {
                    // 3J, 1, 1 or 3, 1, 1J
                    HandType::FourOfAKind
                } else {
                    // 3, 1, 1
                    HandType::ThreeOfAKind
                }
            },
            // 2, 2, 1J
            3 if card_counts[0].0 == &Card::Joker => HandType::FullHouse,
            // 2J, 2, 1
            3 if contains_joker => HandType::FourOfAKind,
            // 2, 2, 1
            3 => HandType::TwoPair,
            // 2J, 1, 1, 1 or 2, 1, 1, 1J
            4 if contains_joker => HandType::ThreeOfAKind,
            // 2, 1, 1, 1
            4 => HandType::Pair,
            // 1J, 1, 1, 1, 1
            _ if contains_joker => HandType::Pair,
            // 1, 1, 1, 1, 1
            _ => HandType::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_types_ord = self.hand_type().cmp(&other.hand_type());

        match hand_types_ord {
            Ordering::Equal => self
                .cards
                .iter()
                .zip(&other.cards)
                .map(|(a, b)| a.cmp(b))
                .find(|&ordering| ordering != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            _ => hand_types_ord,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub hand: Hand,
    pub bid: u64,
}

impl Game {
    pub fn new(hand: Hand, bid: u64) -> Self {
        Self { hand, bid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_from_char() {
        assert_eq!('A'.try_into(), Ok(Card::Ace));
        assert_eq!('K'.try_into(), Ok(Card::King));
        assert_eq!('T'.try_into(), Ok(Card::Ten));
        assert_eq!('2'.try_into(), Ok(Card::Two));
    }

    #[test]
    fn card_order() {
        assert!(Card::Ace > Card::King);
        assert!(Card::Queen >= Card::Three);
        assert!(Card::Jack == Card::Jack);
        assert!(Card::Two <= Card::Two);
        assert!(Card::Two < Card::Six);
    }

    #[test]
    fn hand_type() {
        assert_eq!(
            Hand::new([Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace]).hand_type(),
            HandType::FiveOfAKind
        );
        assert_eq!(
            Hand::new([Card::Ace, Card::Ace, Card::Eight, Card::Ace, Card::Ace]).hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::new([Card::Two, Card::Three, Card::Three, Card::Three, Card::Two]).hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand::new([Card::Ten, Card::Ten, Card::Ten, Card::Nine, Card::Eight]).hand_type(),
            HandType::ThreeOfAKind
        );
        assert_eq!(
            Hand::new([Card::Two, Card::Three, Card::Four, Card::Three, Card::Two]).hand_type(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand::new([Card::Ace, Card::Two, Card::Three, Card::Ace, Card::Four]).hand_type(),
            HandType::Pair
        );
        assert_eq!(
            Hand::new([Card::Two, Card::Three, Card::Four, Card::Five, Card::Six]).hand_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn hand_type_order() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
        assert!(HandType::FourOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::ThreeOfAKind);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::TwoPair > HandType::Pair);
        assert!(HandType::Pair > HandType::HighCard);
    }

    #[test]
    fn hand_order() {
        let hand1 = Hand::new([Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]);
        let hand2 = Hand::new([Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]);
        let hand3 = Hand::new([Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]);

        assert!(hand1 < hand2);
        assert!(hand1 < hand3);
        assert!(hand2 > hand3);
    }

    #[test]
    fn hand_type_with_joker() {
        assert_eq!(
            Hand::new([Card::Ten, Card::Five, Card::Five, Card::Joker, Card::Five]).hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::new([Card::King, Card::Ten, Card::Joker, Card::Joker, Card::Ten]).hand_type(),
            HandType::FourOfAKind
        );
        assert_eq!(
            Hand::new([Card::Queen, Card::Queen, Card::Queen, Card::Joker, Card::Ace]).hand_type(),
            HandType::FourOfAKind
        );
    }
}
