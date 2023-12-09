use std::fmt::Display;
use tokenizer::Parser;

fn main() {
    let data = include_str!("../data.txt");
    println!("part 1: {}", part1(data));
    println!("part 2: {}", part2(data));
}

fn part1(data: &str) -> impl Display {
    let sequence_of_sequences = parse_data(data);
    let mut results = Vec::new();

    for sequence in sequence_of_sequences {
        let mut sequence_stack = Vec::new();
        sequence_stack.push(sequence);

        let mut all_0s = false;

        while !all_0s {
            all_0s = true;
            let mut new_sequence = Vec::new();

            for slice in sequence_stack
                .get(sequence_stack.len() - 1)
                .unwrap()
                .windows(2)
            {
                let new_val = slice[1] - slice[0];
                if new_val != 0 {
                    all_0s = false;
                }
                new_sequence.push(new_val);
            }

            sequence_stack.push(new_sequence);
        }

        let mut previous = None;
        for sub_sequence in sequence_stack.into_iter().rev().skip(1) {
            previous = Some(sub_sequence[sub_sequence.len() - 1] + previous.unwrap_or(0));
        }

        results.push(previous.unwrap())
    }

    results.into_iter().sum::<isize>()
}

fn part2(data: &str) -> impl Display {
    let sequence_of_sequences = parse_data(data);
    let mut results = Vec::new();

    for sequence in sequence_of_sequences {
        let mut sequence_stack = Vec::new();
        sequence_stack.push(sequence);

        let mut all_0s = false;

        while !all_0s {
            all_0s = true;
            let mut new_sequence = Vec::new();

            for slice in sequence_stack
                .get(sequence_stack.len() - 1)
                .unwrap()
                .windows(2)
            {
                let new_val = slice[1] - slice[0];
                if new_val != 0 {
                    all_0s = false;
                }
                new_sequence.push(new_val);
            }

            sequence_stack.push(new_sequence);
        }

        let mut previous = None;
        for sub_sequence in sequence_stack.into_iter().rev().skip(1) {
            previous = Some(sub_sequence[0] - previous.unwrap_or(0));
        }

        results.push(previous.unwrap())
    }

    results.into_iter().sum::<isize>()
}

fn parse_data(data: &str) -> Vec<Vec<isize>> {
    let mut results = Vec::new();

    for line in data.lines() {
        let mut parser = Parser::new(line);
        let mut line_result = Vec::new();

        loop {
            parser.match_char_while(|ch| ch == ' ');
            line_result.push(parser.match_int().expect("To see int").1);

            if parser.peek_position() == line.len() {
                break;
            }
        }
        results.push(line_result);
    }

    results
}
