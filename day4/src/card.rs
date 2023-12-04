
pub struct ScratchCard {
    pub id: usize,
    pub winning_numbers: Vec<u32>,
    pub numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn new(id: usize, winning_numbers: Vec<u32>, numbers: Vec<u32>) -> Self {
        Self { id, winning_numbers, numbers }
    }

    pub fn score(&self) -> u32 {
        let winning_count = self.numbers
            .iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .count() as u32;

        if winning_count == 0 {
            0
        } else {
            2u32.pow(winning_count - 1)
        }
    }

    pub fn won_cards(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .count() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scratchcard_score() {
        let card = ScratchCard::new(
            1,
            vec![1, 2, 3],
            vec![5, 4, 3, 2],
        );

        assert_eq!(card.score(), 2);

        let card = ScratchCard::new(
            1,
            vec![1, 2, 3],
            vec![5, 6, 7],
        );

        assert_eq!(card.score(), 0);

        let card = ScratchCard::new(
            1,
            vec![1, 2, 3],
            vec![5, 4, 1],
        );

        assert_eq!(card.score(), 1);

        let card = ScratchCard::new(
            1,
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        );

        assert_eq!(card.score(), 8);


        let card = ScratchCard::new(
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        );

        assert_eq!(card.score(), 8);
    }
}
