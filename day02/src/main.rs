use std::{fmt::Display, iter::Peekable, str::Chars};

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
                let power = game
                    .rounds
                    .iter()
                    .map(|round| round.red)
                    .max()
                    .expect("Game to have at least 1 round")
                    * game
                        .rounds
                        .iter()
                        .map(|round| round.blue)
                        .max()
                        .expect("Game to have at least 1 round")
                    * game
                        .rounds
                        .iter()
                        .map(|round| round.green)
                        .max()
                        .expect("Game to have at least 1 round");

                result += power;
            }
        }
    }

    return result;
}

fn parse_input(data: &str) -> Result<Vec<Game>, &'static str> {
    let mut games = Vec::new();

    for line in data.lines() {
        let mut parser = Parser::new(line);
        if !parser.match_str("Game ") {
            return Err("Game");
        }

        let game_num;
        match parser.match_int() {
            None => return Err("Game num"),
            Some(int) => game_num = int,
        }

        if !parser.match_str(": ") {
            return Err(":");
        }

        let mut game = Game {
            number: game_num,
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
                    Some(int) => {
                        parser.match_char(' ');
                        if parser.match_str("red") {
                            red = Some(int);
                        } else if parser.match_str("blue") {
                            blue = Some(int);
                        } else if parser.match_str("green") {
                            green = Some(int);
                        } else {
                            return Err("red, blue, or green");
                        }
                    }
                }

                parser.match_str(", ");
                if parser.match_str("; ") {
                    break;
                }
            }

            game.rounds.push(Round {
                red: red.unwrap_or(0),
                blue: blue.unwrap_or(0),
                green: green.unwrap_or(0),
            });

            if parser.chars_iter.peek().is_none() {
                break;
            }
        }

        games.push(game);
    }

    return Ok(games);
}

struct Parser<'i> {
    source: &'i str,
    chars_iter: Peekable<Chars<'i>>,
}

impl<'i> Parser<'i> {
    fn new(source: &'i str) -> Self {
        Self {
            source,
            chars_iter: source.chars().peekable(),
        }
    }

    fn match_str(&mut self, str: &str) -> bool {
        for char in str.chars() {
            if !self.match_char(char) {
                return false;
            }
        }
        return true;
    }

    fn match_char(&mut self, ch: char) -> bool {
        match self.chars_iter.peek() {
            None => false,
            Some(source_ch) => {
                if *source_ch == ch {
                    self.chars_iter.next();
                    return true;
                }
                return false;
            }
        }
    }

    fn match_int(&mut self) -> Option<isize> {
        let mut num_str = String::new();
        if self.match_char('-') {
            num_str.push('-');
        }
        loop {
            match self.chars_iter.peek() {
                None => break,
                Some(ch) => match ch {
                    '0'..='9' => {
                        num_str.push(*ch);
                        self.chars_iter.next();
                    }
                    _ => break,
                },
            }
        }

        if num_str.len() == 0 || num_str.len() == 1 && num_str[0..1] == *"-" {
            return None;
        }

        return Some(
            num_str
                .parse::<isize>()
                .expect("string to be a valid integer at this point."),
        );
    }
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
