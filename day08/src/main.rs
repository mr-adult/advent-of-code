use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use tokenizer::Parser;

fn main() {
    let data = include_str!("../data.txt");
    println!("part 1: {}", part1(data));
    println!("part 2: {}", part2(data));
}

fn part1(data: &str) -> impl Display {
    let (directions, nodes) = parse_input(data);
    let mut current_location = "AAA".to_string();
    let mut current = nodes.get("AAA").expect("AAA to exist");

    let mut num_steps = 0;
    loop {
        if current_location == "ZZZ" {
            break;
        }
        for direction in directions.iter() {
            num_steps += 1;
            match direction {
                Direction::L => {
                    current_location = current[0].to_string();
                    current = nodes.get(&current[0]).expect("left to be defined")
                }
                Direction::R => {
                    current_location = current[1].to_string();
                    current = nodes.get(&current[1]).expect("right to be defined")
                }
            }
        }
    }

    num_steps
}

fn part2(data: &str) -> impl Display {
    let (directions, nodes) = parse_input(data);

    let current_locations = nodes
        .iter()
        .filter(|node| node.0.chars().nth(2).unwrap() == 'A')
        .map(|node| node.0.to_string())
        .collect::<Vec<_>>();

    let combined = current_locations
        .into_iter()
        .map(|current| {
            (
                current.to_string(),
                nodes.get(&current).expect("node to exist"),
            )
        })
        .collect::<Vec<_>>();

    println!("calculating steps for each");
    let num_steps_list = combined
        .into_iter()
        .par_bridge()
        .map(|(mut current_location, mut current)| {
            let mut num_steps = 0;
            loop {
                if current_location.chars().nth(2).unwrap() == 'Z' {
                    break;
                }
                for direction in directions.iter() {
                    num_steps += 1;
                    match direction {
                        Direction::L => {
                            current_location = current[0].to_string();
                            current = nodes.get(&current[0]).expect("left to be defined")
                        }
                        Direction::R => {
                            current_location = current[1].to_string();
                            current = nodes.get(&current[1]).expect("right to be defined")
                        }
                    }
                }
            }

            num_steps
        })
        .collect::<Vec<_>>();

    println!("calculating LCM");
    least_common_multiple(num_steps_list)
}

fn parse_input(data: &str) -> (Vec<Direction>, HashMap<String, [String; 2]>) {
    let mut directions = Vec::new();
    let mut map = HashMap::new();

    for (line_num, line) in data.lines().enumerate() {
        let mut parser = Parser::new(line);
        if line_num == 0 {
            loop {
                if parser.match_char('R').is_some() {
                    directions.push(Direction::R);
                } else if parser.match_char('L').is_some() {
                    directions.push(Direction::L);
                } else if parser.peek_position() == line.len() {
                    break;
                } else {
                    panic!("Expected L or R at {:?}", parser.peek_position());
                }
            }
        } else if line_num == 1 {
            continue;
        } else {
            let span = parser
                .match_char_while(|ch| match ch {
                    'A'..='Z' => true,
                    _ => false,
                })
                .unwrap();

            let id = line[span.start()..span.end()].to_string();

            parser.match_char_while(|ch| ch == ' ');
            parser.match_char('=');
            parser.match_char_while(|ch| ch == ' ');
            parser.match_char('(');

            let span = parser
                .match_char_while(|ch| match ch {
                    'A'..='Z' => true,
                    _ => false,
                })
                .unwrap();
            let left = line[span.start()..span.end()].to_string();

            parser.match_char_while(|ch| ch == ' ');
            parser.match_char(',');
            parser.match_char_while(|ch| ch == ' ');

            let span = parser
                .match_char_while(|ch| match ch {
                    'A'..='Z' => true,
                    _ => false,
                })
                .unwrap();
            let right = line[span.start()..span.end()].to_string();

            parser.match_char(')');
            parser.match_char_while(|ch| ch == ' ');

            if map.insert(id.clone(), [left, right]).is_some() {
                panic!("Duplicate key {}", id);
            }
        }
    }

    (directions, map)
}

enum Direction {
    L,
    R,
}

fn least_common_multiple(nums: Vec<usize>) -> usize {
    let mut lcm_so_far = 1;
    for num in nums {
        lcm_so_far = least_common_multiple_pair(lcm_so_far, num);
    }
    lcm_so_far
}

fn least_common_multiple_pair(num1: usize, num2: usize) -> usize {
    let mut primes1 = prime_factorization(num1).into_iter();
    let mut primes2 = prime_factorization(num2).into_iter();

    let mut current1 = primes1.next();
    let mut current2 = primes2.next();

    let mut result = Vec::new();

    loop {
        if current1.is_some() && current2.is_some() {
            match current1
                .as_ref()
                .unwrap()
                .prime
                .cmp(&current2.as_ref().unwrap().prime)
            {
                Ordering::Less => {
                    result.push(current1.unwrap());
                    current1 = primes1.next();
                }
                Ordering::Equal => {
                    let factor1 = current1.unwrap();
                    let prime = factor1.prime;
                    result.push(Factor {
                        prime,
                        power: [factor1.power, current2.unwrap().power]
                            .into_iter()
                            .max()
                            .unwrap(),
                    });

                    current1 = primes1.next();
                    current2 = primes2.next();
                }
                Ordering::Greater => {
                    result.push(current2.unwrap());
                    current2 = primes2.next();
                }
            }
        } else if let Some(current1_val) = current1 {
            result.push(current1_val);
            current1 = primes1.next();
        } else if let Some(current2_val) = current2 {
            result.push(current2_val);
            current2 = primes2.next();
        } else {
            break;
        }
    }

    result
        .into_iter()
        .fold(1, |acc, factor| acc * factor.prime.pow(factor.power as u32))
}

fn prime_factorization(mut num: usize) -> Vec<Factor> {
    if num == 0 {
        return Vec::new();
    }

    let mut primes = Vec::new();
    let mut factors = Vec::new();

    for i in 2.. {
        let mut is_prime = true;
        for prime in primes.iter() {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if !is_prime {
            continue;
        }
        primes.push(i);

        let mut power = 0;
        while num % i == 0 {
            power += 1;
            num /= i;
        }

        factors.push(Factor { power, prime: i });

        if num == 1 {
            if factors.len() == 1 {
                factors.push(Factor { power: 0, prime: 2 })
            }
            break;
        }
    }

    factors
}

#[derive(Debug, PartialEq, Eq)]
struct Factor {
    power: usize,
    prime: usize,
}
