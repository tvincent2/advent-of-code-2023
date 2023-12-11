use std::ops::Deref;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord)]
enum Card {
    A,
    K,
    Q,
    T,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
    J,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::N9,
            '8' => Card::N8,
            '7' => Card::N7,
            '6' => Card::N6,
            '5' => Card::N5,
            '4' => Card::N4,
            '3' => Card::N3,
            '2' => Card::N2,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    FiveOfAKind(Card, Card, Card, Card, Card),
    FourOfAKind(Card, Card, Card, Card, Card),
    FullHouse(Card, Card, Card, Card, Card),
    ThreeOfAKind(Card, Card, Card, Card, Card),
    TwoPair(Card, Card, Card, Card, Card),
    OnePair(Card, Card, Card, Card, Card),
    HighCard(Card, Card, Card, Card, Card),
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        let cards: Vec<Card> = input.chars().map(|c| Card::from(c)).collect();
        let mut sorted_cards = cards.clone();
        sorted_cards.sort();

        match sorted_cards[..] {
            // five of a kind
            [c1, _, _, _, c5] if c1 == c5 => {
                Hand::FiveOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
            }
            // four of a kind
            [c1, _, _, c4, c5] if c1 == c4 => {
                if c5 == Card::J {
                    Hand::FiveOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [_, c2, _, _, c5] if c2 == c5 => {
                if c5 == Card::J {
                    Hand::FiveOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            // full house
            [c1, _, c3, c4, c5] if c1 == c3 && c4 == c5 => {
                if c5 == Card::J {
                    Hand::FiveOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [c1, c2, c3, _, c5] if c1 == c2 && c3 == c5 => {
                if c5 == Card::J {
                    Hand::FiveOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            // three of a kind
            [c1, _, c3, _, c5] if c1 == c3 => {
                if c5 == Card::J {
                    Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [_, c2, _, c4, c5] if c2 == c4 => {
                if c5 == Card::J {
                    Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [_, _, c3, _, c5] if c3 == c5 => {
                if c5 == Card::J {
                    Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            // two pair
            [c1, c2, c3, c4, c5] if c1 == c2 && c3 == c4 => {
                if c5 == Card::J {
                    Hand::FullHouse(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::TwoPair(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [c1, c2, _, c4, c5] if c1 == c2 && c4 == c5 => {
                if c5 == Card::J {
                    Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::TwoPair(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [_, c2, c3, c4, c5] if c2 == c3 && c4 == c5 => {
                if c5 == Card::J {
                    Hand::FourOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::TwoPair(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            // one pair
            [c1, c2, _, _, c5] if c1 == c2 => {
                if c5 == Card::J {
                    Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [_, c2, c3, _, c5] if c2 == c3 => {
                if c5 == Card::J {
                    Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [_, _, c3, c4, c5] if c3 == c4 => {
                if c5 == Card::J {
                    Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            [_, _, _, c4, c5] if c4 == c5 => {
                if c5 == Card::J {
                    Hand::ThreeOfAKind(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            // high card
            [_, _, _, _, c5] => {
                if c5 == Card::J {
                    Hand::OnePair(cards[0], cards[1], cards[2], cards[3], cards[4])
                } else {
                    Hand::HighCard(cards[0], cards[1], cards[2], cards[3], cards[4])
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Bid {
    bid: usize,
    hand: Hand,
}

impl From<&str> for Bid {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(" ").collect();
        let hand = Hand::from(split[0]);
        let bid = split[1].parse::<usize>().expect("unparseable bid");
        Bid { bid, hand }
    }
}

#[derive(Debug, PartialEq)]
struct Bids {
    bids: Vec<Bid>,
}

impl Bids {
    fn total_winnings(&self) -> usize {
        self.bids
            .iter()
            .enumerate()
            .map(|(index, bid)| (index + 1) * bid.bid)
            .sum()
    }
}

impl Deref for Bids {
    type Target = Vec<Bid>;

    fn deref(&self) -> &Self::Target {
        &self.bids
    }
}

impl From<&str> for Bids {
    fn from(value: &str) -> Self {
        let mut bids: Vec<Bid> = value.lines().map(|line| Bid::from(line)).collect();
        bids.sort_by(|a, b| b.hand.cmp(&a.hand));
        Bids { bids }
    }
}

fn main() {
    let input = include_str!("../../input/day-07");
    let bids = Bids::from(input);
    let total_winnings = bids.total_winnings();
    println!("Total winnings: {}", total_winnings);
}

#[cfg(test)]
mod tests {
    use crate::{Bids, Card, Hand};

    #[test]
    fn card_parsing() {
        assert_eq!(
            Hand::from("32T3K"),
            Hand::OnePair(Card::N3, Card::N2, Card::T, Card::N3, Card::K)
        );
        assert_eq!(
            Hand::from("T55J5"),
            Hand::ThreeOfAKind(Card::T, Card::N5, Card::N5, Card::J, Card::N5)
        );
        assert_eq!(
            Hand::from("KK677"),
            Hand::TwoPair(Card::K, Card::K, Card::N6, Card::N7, Card::N7)
        );
        assert_eq!(
            Hand::from("KTJJT"),
            Hand::TwoPair(Card::K, Card::T, Card::J, Card::J, Card::T)
        );
        assert_eq!(
            Hand::from("QQQJA"),
            Hand::ThreeOfAKind(Card::Q, Card::Q, Card::Q, Card::J, Card::A)
        );
        assert_eq!(
            Hand::from("86452"),
            Hand::HighCard(Card::N8, Card::N6, Card::N4, Card::N5, Card::N2)
        );
    }

    #[test]
    fn card_parsing_step2() {
        assert_eq!(
            Hand::from("32T3K"),
            Hand::OnePair(Card::N3, Card::N2, Card::T, Card::N3, Card::K)
        );
        assert_eq!(
            Hand::from("T55J5"),
            Hand::FourOfAKind(Card::T, Card::N5, Card::N5, Card::J, Card::N5)
        );
        assert_eq!(
            Hand::from("KK677"),
            Hand::TwoPair(Card::K, Card::K, Card::N6, Card::N7, Card::N7)
        );
        assert_eq!(
            Hand::from("KTJJT"),
            Hand::FourOfAKind(Card::K, Card::T, Card::J, Card::J, Card::T)
        );
        assert_eq!(
            Hand::from("QQQJA"),
            Hand::FourOfAKind(Card::Q, Card::Q, Card::Q, Card::J, Card::A)
        );
        assert_eq!(
            Hand::from("86452"),
            Hand::HighCard(Card::N8, Card::N6, Card::N4, Card::N5, Card::N2)
        );
    }

    #[test]
    fn total_winnings() {
        let input = include_str!("../../input/day-07-test");
        let bids = Bids::from(input);
        assert_eq!(bids.total_winnings(), 6440);
    }

    #[test]
    fn total_winnings_step2() {
        let input = include_str!("../../input/day-07-test");
        let bids = Bids::from(input);
        assert_eq!(bids.total_winnings(), 5905);
    }

    #[test]
    fn total_winnings_other_input() {
        let input = include_str!("../../input/day-07-other-test");
        let bids = Bids::from(input);
        assert_eq!(bids.total_winnings(), 6592);
    }

    #[test]
    fn total_winnings_other_input_step2() {
        let input = include_str!("../../input/day-07-other-test");
        let bids = Bids::from(input);
        assert_eq!(bids.total_winnings(), 6592);
    }

    #[test]
    fn card_order() {
        assert!(Card::A > Card::J);
    }

    #[test]
    fn hand_order() {
        assert!(
            Hand::HighCard(Card::A, Card::J, Card::N5, Card::N4, Card::N3)
                > Hand::HighCard(Card::A, Card::N5, Card::N4, Card::N3, Card::N2)
        );
    }

    #[test]
    fn hand_order2() {
        assert!(Hand::from("77888") < Hand::from("77788"));
    }

    #[test]
    fn hand_order_with_joker() {
        assert!(dbg!(Hand::from("J367J")) < dbg!(Hand::from("J3749")));
    }

    #[test]
    fn hand_order_with_joker2() {
        assert!(dbg!(Hand::from("JK6AA")) > dbg!(Hand::from("JKJ5J")));
    }
}
