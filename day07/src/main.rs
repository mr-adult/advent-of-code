use core::num;
use std::{cmp::Ordering, fmt::Display};
use tokenizer::Parser;

fn main() {
    let data = include_str!("../data.txt");
    println!("part 1: {}", part1(data));
    println!("part 2: {}", part2(data));
}

fn part1(data: &str) -> impl Display {
    let mut hands = parse_input(data);
    hands.sort_by(|hand1, hand2| {
        let hand_cmp = hand1.hand_type().cmp(&hand2.hand_type());
        if hand_cmp != Ordering::Equal {
            return hand_cmp;
        }

        for i in 0..5 {
            let card_cmp = hand1.cards[i].cmp(&hand2.cards[i]);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }

        return Ordering::Equal;
    });

    let mut total = 0;
    for (rank, hand) in hands.into_iter().enumerate() {
        println!("hand type: {:?}. Hand: {:?}", hand.hand_type(), hand);
        total += (rank + 1) * hand.bid;
    }

    total
}

fn part2(data: &str) -> impl Display {
    let mut hands = parse_input(data);

    hands.sort_by(|hand1, hand2| {
        let hand_cmp = hand1
            .hand_type_with_joker()
            .cmp(&hand2.hand_type_with_joker());

        if hand_cmp != Ordering::Equal {
            return hand_cmp;
        }

        for i in 0..5 {
            if hand1.cards[i] == Card::J && hand2.cards[i] == Card::J {
                continue;
            } else if hand1.cards[i] == Card::J {
                return Ordering::Less;
            } else if hand2.cards[i] == Card::J {
                return Ordering::Greater;
            }

            let card_cmp = hand1.cards[i].cmp(&hand2.cards[i]);
            if card_cmp != Ordering::Equal {
                return card_cmp;
            }
        }

        return Ordering::Equal;
    });

    let mut total = 0;
    for (rank, hand) in hands.into_iter().enumerate() {
        println!(
            "hand type: {:?}. Hand: {:?}",
            hand.hand_type_with_joker(),
            hand
        );
        total += (rank + 1) * hand.bid;
    }

    total
}

fn parse_input(data: &str) -> Vec<Hand> {
    let mut hands = Vec::new();

    for line in data.lines() {
        let mut parser = Parser::new(line);
        let mut cards = [Card::default(); 5];
        for i in 0..5 {
            parser.match_char_if(|ch| match ch {
                '2' => {
                    cards[i] = Card::Two;
                    true
                }
                '3' => {
                    cards[i] = Card::Three;
                    true
                }
                '4' => {
                    cards[i] = Card::Four;
                    true
                }
                '5' => {
                    cards[i] = Card::Five;
                    true
                }
                '6' => {
                    cards[i] = Card::Six;
                    true
                }
                '7' => {
                    cards[i] = Card::Seven;
                    true
                }
                '8' => {
                    cards[i] = Card::Eight;
                    true
                }
                '9' => {
                    cards[i] = Card::Nine;
                    true
                }
                'T' => {
                    cards[i] = Card::T;
                    true
                }
                'J' => {
                    cards[i] = Card::J;
                    true
                }
                'Q' => {
                    cards[i] = Card::Q;
                    true
                }
                'K' => {
                    cards[i] = Card::K;
                    true
                }
                'A' => {
                    cards[i] = Card::A;
                    true
                }
                _ => panic!("Unexpected card: {}", ch),
            });
        }

        parser.match_char_while(|ch| ch == ' ');

        let bid = parser.match_uint().expect("bid to be usize").1;
        hands.push(Hand { bid, cards })
    }

    hands
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    bid: usize,
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut card_counts = Vec::new();
        for card in self.cards.iter() {
            let matched = card_counts
                .iter_mut()
                .find(|(_, counted_card)| *card == *counted_card);

            match matched {
                None => {
                    card_counts.push((1, *card));
                }
                Some(card_with_count) => {
                    card_with_count.0 += 1;
                }
            }
        }

        card_counts.sort_by(|item1, item2| item2.0.cmp(&item1.0));
        match card_counts[0] {
            (5, card) => HandType::FiveOfKind(card),
            (4, card) => HandType::FourOfKind(card),
            (3, major_card) => match card_counts[1] {
                (2, minor_card) => HandType::FullHouse([major_card, minor_card]),
                (1, _) => HandType::ThreeOfKind(major_card),
                _ => panic!("Impossible card combination. Shouldn't get here!"),
            },
            (2, major_card) => match card_counts[1] {
                (2, minor_card) => HandType::TwoPair([major_card, minor_card]),
                (1, _) => HandType::OnePair(major_card),
                (num, other_card) => panic!(
                    "Impossible card combination. Shouldn't get here! Found 2 of {:?} and {} of {:?}",
                    major_card, num, other_card
                ),
            },
            (1, _) => HandType::HighCard(*self.cards.iter().max().unwrap()),
            _ => {
                panic!("Impossible card combination. Shouldn't get here!")
            }
        }
    }

    fn hand_type_with_joker(&self) -> HandType {
        let mut num_jokers = 0;
        let mut card_counts = Vec::new();
        for card in self.cards.iter() {
            let matched = card_counts
                .iter_mut()
                .find(|(_, counted_card)| *card == *counted_card);

            match matched {
                None => {
                    if *card == Card::J {
                        num_jokers += 1;
                        continue;
                    }
                    card_counts.push((1, *card));
                }
                Some(card_with_count) => {
                    card_with_count.0 += 1;
                }
            }
        }

        if num_jokers == 5 {
            return HandType::FiveOfKind(Card::A);
        }

        card_counts.sort_by(|item1, item2| item2.0.cmp(&item1.0));
        match card_counts[0] {
            (5, card) => HandType::FiveOfKind(card),
            (4, card) => {
                if num_jokers == 1 {
                    HandType::FiveOfKind(card)
                } else {
                    HandType::FourOfKind(card)
                }
            }
            (3, major_card) => {
                if num_jokers == 2 {
                    HandType::FiveOfKind(major_card)
                } else if num_jokers == 1 {
                    HandType::FourOfKind(major_card)
                } else {
                    match card_counts[1] {
                        (2, minor_card) => HandType::FullHouse([major_card, minor_card]),
                        (1, _) => HandType::ThreeOfKind(major_card),
                        _ => panic!("Impossible card combination. Shouldn't get here!"),
                    }
                }
            }
            (2, major_card) => {
                if num_jokers == 3 {
                    HandType::FiveOfKind(major_card)
                } else if num_jokers == 2 {
                    HandType::FourOfKind(major_card)
                } else if num_jokers == 1 {
                    if card_counts[1].0 == 2 {
                        HandType::FullHouse([major_card, card_counts[1].1])
                    } else {
                        HandType::ThreeOfKind(major_card)
                    }
                } else {
                    match card_counts[1] {
                        (2, minor_card) => HandType::TwoPair([major_card, minor_card]),
                        (1, _) => HandType::OnePair(major_card),
                        (num, other_card) => panic!(
                            "Impossible card combination. Shouldn't get here! Found 2 of {:?} and {} of {:?}",
                            major_card, num, other_card
                        ),
                    }
                }
            }
            (1, _) => {
                let non_joker_max = *self
                    .cards
                    .iter()
                    .filter(|card| **card != Card::J)
                    .max()
                    .unwrap();

                match num_jokers {
                    4 => HandType::FiveOfKind(non_joker_max),
                    3 => HandType::FourOfKind(non_joker_max),
                    2 => HandType::ThreeOfKind(non_joker_max),
                    1 => HandType::OnePair(non_joker_max),
                    _ => HandType::HighCard(*self.cards.iter().max().unwrap()),
                }
            }
            _ => {
                panic!("Impossible card combination. Shouldn't get here!")
            }
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum HandType {
    HighCard(Card),
    OnePair(Card),
    TwoPair([Card; 2]),
    ThreeOfKind(Card),
    FullHouse([Card; 2]),
    FourOfKind(Card),
    FiveOfKind(Card),
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let index_cmp = self.index().cmp(&other.index());
        return Some(index_cmp);
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl HandType {
    fn index(&self) -> usize {
        match self {
            Self::HighCard(_) => 0,
            Self::OnePair(_) => 1,
            Self::TwoPair(_) => 2,
            Self::ThreeOfKind(_) => 3,
            Self::FullHouse(_) => 4,
            Self::FourOfKind(_) => 5,
            Self::FiveOfKind(_) => 6,
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Default)]
enum Card {
    #[default]
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[cfg(test)]
mod tests {

    #[test]
    fn basic() {
        let input = "22J33 120\n222QJ 80";
        assert_eq!(format!("{}", 280), format!("{}", super::part2(input)));
    }
}
