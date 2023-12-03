use std::fmt::Display;
use tokenizer::Parser;

fn main() {
    let data = include_str!("../data.txt");
    let bag_contents = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    println!("part 1: {}", part1(data, &bag_contents));
    println!("part 2: {}", part2(data, &bag_contents));
}

fn part1(data: &str, bag_contents: &Round) -> impl Display {
    let mut total = 0;
    match parse_input(data) {
        Err(message) => {
            panic!("Failed to parse input. Message: {}", message);
        }
        Ok(games) => {
            for game in games {
                if game.rounds.into_iter().any(|round| {
                    round.red > bag_contents.red
                        || round.blue > bag_contents.blue
                        || round.green > bag_contents.green
                }) {
                    continue;
                }
                total += game.number;
            }
        }
    }
    return total;
}

fn part2(data: &str, bag_contents: &Round) -> impl Display {
    let mut result = 0;
    match parse_input(data) {
        Err(message) => {
            panic!("Failed to parse input. Message: {}", message);
        }
        Ok(games) => {
            for game in games {
                let maxes = game.rounds.iter().fold(
                    Round {
                        red: 0,
                        blue: 0,
                        green: 0,
                    },
                    |mut acc, round| {
                        if round.red > acc.red {
                            acc.red = round.red;
                        }
                        if round.blue > acc.blue {
                            acc.blue = round.blue;
                        }
                        if round.green > acc.green {
                            acc.green = round.green;
                        }
                        acc
                    },
                );

                result += maxes.red * maxes.blue * maxes.green;
            }
        }
    }

    return result;
}

fn parse_input(data: &str) -> Result<Vec<Game>, &'static str> {
    let mut games = Vec::new();

    for line in data.lines() {
        let mut parser = Parser::new(line);
        if parser.match_str("Game ").is_none() {
            return Err("Game");
        }

        let game_num;
        match parser.match_int() {
            None => return Err("Game num"),
            Some(int) => game_num = int,
        }

        if parser.match_str(": ").is_none() {
            return Err(":");
        }

        let mut game = Game {
            number: game_num.1,
            rounds: Vec::new(),
        };

        loop {
            let mut red = None;
            let mut green = None;
            let mut blue = None;

            for _ in 0..3 {
                match parser.match_int() {
                    None => {
                        break;
                    }
                    Some((_, int)) => {
                        parser.match_char(' ');
                        if parser.match_str("red").is_some() {
                            red = Some(int);
                        } else if parser.match_str("blue").is_some() {
                            blue = Some(int);
                        } else if parser.match_str("green").is_some() {
                            green = Some(int);
                        } else {
                            return Err("red, blue, or green");
                        }
                    }
                }

                parser.match_str(", ");
                if parser.match_str("; ").is_some() {
                    break;
                }
            }

            game.rounds.push(Round {
                red: red.unwrap_or(0),
                blue: blue.unwrap_or(0),
                green: green.unwrap_or(0),
            });

            if parser.peek().is_none() {
                break;
            }
        }

        games.push(game);
    }

    return Ok(games);
}

struct Game {
    number: isize,
    rounds: Vec<Round>,
}

struct Round {
    red: isize,
    blue: isize,
    green: isize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn cases() {
        let bag_contents = super::Round {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert_eq!(
            format!("{}", 2512),
            format!(
                "{}",
                super::part1(include_str!("../data.txt"), &bag_contents)
            )
        );
        assert_eq!(
            format!("{}", 67335),
            format!(
                "{}",
                super::part2(include_str!("../data.txt"), &bag_contents)
            )
        );
    }
}
