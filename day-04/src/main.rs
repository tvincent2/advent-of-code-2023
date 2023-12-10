use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn get_number_of_wins(&self) -> usize {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count()
    }

    fn get_points(&self) -> usize {
        match self.get_number_of_wins() {
            0 => 0,
            n => {
                let base: usize = 2;
                base.pow((n - 1) as u32)
            }
        }
    }
}

impl From<&str> for Card {
    fn from(input: &str) -> Self {
        let split1: Vec<&str> = input.split(":").collect();
        let card_id_str = split1[0];
        let numbers_str = split1[1];
        let id = card_id_str
            .split(" ")
            .last()
            .expect("missing id number")
            .parse::<usize>()
            .expect("id not parseable");
        let split2: Vec<&str> = numbers_str.split("|").collect();
        let winning_numbers = split2[0]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().expect("winning number not parseable"))
            .collect();
        let numbers = split2[1]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().expect("winning number not parseable"))
            .collect();
        Card {
            id,
            winning_numbers,
            numbers,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cards {
    cards: Vec<Card>,
}

impl Cards {
    fn get_points(&self) -> usize {
        self.iter().map(|card| card.get_points()).sum()
    }

    fn compute_sum_of_won_cards(&self) -> usize {
        let mut numbers_of_cards = vec![1; self.cards.len()];
        self.cards
            .iter()
            .enumerate()
            .for_each(|(card_index, card)| {
                let number_of_this_card = numbers_of_cards[card_index];
                let number_of_wins = card.get_number_of_wins();
                for won_card_index in card_index + 1..=card_index + number_of_wins {
                    numbers_of_cards[won_card_index] += number_of_this_card;
                }
            });
        numbers_of_cards.iter().sum()
    }
}

impl Deref for Cards {
    type Target = Vec<Card>;

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl From<&str> for Cards {
    fn from(input: &str) -> Self {
        let cards: Vec<Card> = input.lines().map(|line| Card::from(line)).collect();
        Cards { cards }
    }
}

fn main() {
    let input = include_str!("../../input/day-04");
    let cards = Cards::from(input);
    let sum_of_points = cards.get_points();
    println!("Sum of points: {}", sum_of_points);

    let sum_of_won_cards = cards.compute_sum_of_won_cards();
    println!("Sum of won cards: {}", sum_of_won_cards);
}

#[cfg(test)]
mod tests {
    use crate::{Card, Cards};

    #[test]
    fn read_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from(input);
        assert_eq!(
            card,
            Card {
                id: 1,
                winning_numbers: vec![41, 48, 83, 86, 17],
                numbers: vec![83, 86, 6, 31, 17, 9, 48, 53]
            }
        );
    }

    #[test]
    fn get_card_points() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = Card::from(input);
        let points = card.get_points();
        assert_eq!(points, 8);
    }

    #[test]
    fn get_sum_of_points() {
        let input = include_str!("../../input/day-04-test");
        let cards = Cards::from(input);
        let points = cards.get_points();
        assert_eq!(points, 13);
    }

    #[test]
    fn get_sum_of_won_cards() {
        let input = include_str!("../../input/day-04-test");
        let cards = Cards::from(input);
        let total = cards.compute_sum_of_won_cards();
        assert_eq!(total, 30);
    }
}
