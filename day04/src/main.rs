use std::{collections::VecDeque, fmt::Display};
use tokenizer::Parser;

fn main() {
    let data = include_str!("../data.txt");
    println!("part 1: {}", part1(data));
    println!("part 2: {}", part2(data));
}

fn part1(data: &str) -> impl Display {
    let cards = parse_input(data);
    let mut total = 0;
    for card in cards {
        let mut card_val = 0;
        for actual_num in card.actual_nums {
            if card.winning_nums.contains(&actual_num) {
                if card_val == 0 {
                    card_val = 1;
                } else {
                    card_val *= 2;
                }
            }
        }
        total += card_val
    }

    total
}

fn part2(data: &str) -> impl Display {
    let cards = parse_input(data);
    let mut queue_of_stacks = VecDeque::new();
    let mut resulting_cards = Vec::new();
    for card in cards {
        queue_of_stacks.push_back(vec![card]);
    }

    for i in 0..queue_of_stacks.len() {
        while queue_of_stacks.get(i).expect("stack to be at index").len() > 0 {
            let card = queue_of_stacks
                .get_mut(i)
                .expect("stack to be at index")
                .pop()
                .expect("stack to contain a card");

            let mut card_val = 0;
            for actual_num in card.actual_nums.iter() {
                if card.winning_nums.contains(actual_num) {
                    card_val += 1;
                }
            }

            for j in (i + 1)
                ..([i + card_val + 1, queue_of_stacks.len()]
                    .into_iter()
                    .min()
                    .unwrap() as usize)
            {
                let new_val = queue_of_stacks.get(j).unwrap().get(0).unwrap().clone();
                queue_of_stacks[j].push(new_val);
            }

            resulting_cards.push(card)
        }
    }

    resulting_cards.len()
}

fn parse_input(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let mut parser = Parser::new(line);

        if parser.match_str("Card").is_none() {
            panic!("Line didn't start with 'Card'")
        }
        parser.match_char_while(|ch| ch == ' ');

        let card_num = match parser.match_int() {
            None => panic!("Failed to match card num"),
            Some(num) => num.1,
        };

        if parser.match_char(':').is_none() {
            panic!("No colon after card num");
        };

        let mut winning_nums = Vec::new();
        loop {
            parser.match_char_while(|ch| ch == ' ');

            if parser.match_char('|').is_some() {
                break;
            }

            let num = match parser.match_int() {
                None => panic!("Failed to match number"),
                Some(num) => num,
            };

            winning_nums.push(num.1);
        }

        let mut actual_nums = Vec::new();
        loop {
            parser.match_char_while(|ch| ch == ' ');
            let num = match parser.match_int() {
                None => {
                    if parser.peek_position() == line.len() {
                        break;
                    }
                    panic!("Failed to match winning number");
                }
                Some(num) => num.1,
            };

            actual_nums.push(num);
        }

        cards.push(Card {
            num: card_num,
            actual_nums,
            winning_nums,
        });
    }

    cards
}

#[derive(Debug, Clone)]
struct Card {
    num: isize,
    actual_nums: Vec<isize>,
    winning_nums: Vec<isize>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_actual() {
        let data = include_str!("../data.txt");
        assert_eq!(format!("{}", 21568), format!("{}", super::part1(data)));
    }

    #[test]
    fn part2_actual() {
        let data = include_str!("../data.txt");
        assert_eq!(format!("{}", "unknown"), format!("{}", super::part2(data)));
    }
}
