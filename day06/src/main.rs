use std::fmt::Display;
use tokenizer::Parser;

fn main() {
    let data = include_str!("../data.txt");
    println!("part 1: {}", part1(data));
    println!("part 2: {}", part2(data));
}

fn part1(data: &str) -> impl Display {
    let times_and_distances = process_data(data);

    let mut result = 1;
    for (time, distance) in times_and_distances {
        let mut ways_to_beat = 0;
        for i in 0..=time {
            if i * (time - i) > distance {
                ways_to_beat += 1;
            }
        }

        result *= ways_to_beat;
    }

    result
}

fn part2(data: &str) -> impl Display {
    let times_and_distances = process_data(data);

    let actual_time_and_dist = times_and_distances.into_iter().fold(
        ("".to_string(), "".to_string()),
        |mut acc, time_and_distance| {
            acc.0.push_str(&time_and_distance.0.to_string());
            acc.1.push_str(&time_and_distance.1.to_string());
            acc
        },
    );

    let actual_time_and_dist = (
        actual_time_and_dist.0.parse::<i128>().unwrap(),
        actual_time_and_dist.1.parse::<i128>().unwrap(),
    );

    let mut ways_to_beat = 0;
    for i in 0..=actual_time_and_dist.0 {
        if i * (actual_time_and_dist.0 - i) > actual_time_and_dist.1 {
            ways_to_beat += 1;
        }
    }

    ways_to_beat
}

fn process_data(data: &str) -> Vec<(isize, isize)> {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    for (line_num, mut line_parser) in data.lines().map(|line| Parser::new(line)).enumerate() {
        match line_num {
            0 => {
                line_parser.match_str("Time:").unwrap();
            }
            1 => {
                line_parser.match_str("Distance:").unwrap();
            }
            _ => {
                panic!("Reached line number {}", line_num);
            }
        }
        loop {
            if line_parser.peek_position() == line_parser.source.len() {
                break;
            }
            line_parser.match_char_while(|ch| ch == ' ').unwrap();

            match line_num {
                0 => times.push(line_parser.match_int().unwrap().1),
                1 => distances.push(line_parser.match_int().unwrap().1),
                _ => {
                    panic!();
                }
            }
        }
    }

    times.into_iter().zip(distances).collect()
}
