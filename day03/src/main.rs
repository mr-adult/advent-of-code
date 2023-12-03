use std::{collections::HashMap, fmt::Display};
use tokenizer::{Parser, Span};

fn main() {
    let data = include_str!("../data.txt");
    println!("part 1: {}", part1(data));
    println!("part 2: {}", part2(data));
}

fn part1(data: &str) -> impl Display {
    let mut prev_line_symbols = Vec::with_capacity(0);
    let mut prev_line_parts: Vec<(Span, usize)> = Vec::with_capacity(0);
    let mut valid_parts = HashMap::new();

    for (line_num, line) in data.lines().enumerate() {
        let mut parser = Parser::new(line);
        let mut symbols = Vec::new();
        let mut parts = Vec::new();

        loop {
            // eat all '.'
            parser.match_char_while(|ch| ch == '.');

            if let Some(int_with_span) = parser.match_uint() {
                let span = int_with_span.0;
                let int_start = span.start();
                let int_end = span.end();

                if line[(max(1, int_start) - 1)..min(int_end + 1, line.len())]
                    .chars()
                    .any(is_symbol)
                    || prev_line_symbols
                        .iter()
                        .any(|sym: &Span| sym.start() <= int_end && sym.end() >= int_start)
                {
                    valid_parts.insert((line_num, span), int_with_span.1);
                }

                parts.push(int_with_span);
                continue;
            }

            if let Some(span) = parser.match_char_if(is_symbol) {
                for part in prev_line_parts.iter() {
                    if part.0.end() >= span.start() && part.0.start() <= span.end() {
                        valid_parts.insert((line_num - 1, part.0), part.1);
                    }
                }
                symbols.push(span);
                continue;
            }

            if parser.peek_position() == line.len() {
                break;
            }

            panic!(
                "Unrecognized character at line: {}, column: {}, character: {}",
                line_num,
                parser.peek_position(),
                &line[parser.peek_position()..parser.peek_position() + 1]
            );
        }

        prev_line_parts = parts;
        prev_line_symbols = symbols;
    }

    valid_parts
        .into_iter()
        .map(|kvp| kvp.1)
        .fold(0, |acc, int| acc + int)
}

fn part2(data: &str) -> impl Display {
    let mut prev_line_symbols: Vec<Span> = Vec::with_capacity(0);
    let mut prev_line_parts: Vec<(Span, usize)> = Vec::with_capacity(0);
    let mut parts_by_gear = HashMap::new();

    for (line_num, line) in data.lines().enumerate() {
        let mut parser = Parser::new(line);
        let mut symbols = Vec::new();
        let mut parts = Vec::new();

        loop {
            // eat all '.'
            parser.match_char_while(|ch| ch == '.');

            if let Some(int_with_span) = parser.match_uint() {
                let span = int_with_span.0;
                let int_start = span.start();
                let int_end = span.end();

                let true_start = max(1, int_start) - 1;
                for gear in line[true_start..min(int_end + 1, line.len())]
                    .char_indices()
                    .filter(|ch| ch.1 == '*')
                {
                    parts_by_gear
                        .entry((
                            line_num,
                            Span::new(gear.0 + true_start, gear.0 + true_start + 1),
                        ))
                        .or_insert(Vec::new())
                        .push(int_with_span.1);
                }

                for symbol in prev_line_symbols.iter() {
                    if symbol.start() <= int_end && symbol.end() >= int_start {
                        parts_by_gear
                            .entry((line_num - 1, *symbol))
                            .or_insert(Vec::new())
                            .push(int_with_span.1);
                    }
                }

                parts.push(int_with_span);
                continue;
            }

            if let Some(span) = parser.match_char('*') {
                for part in prev_line_parts.iter() {
                    if part.0.end() >= span.start() && part.0.start() <= span.end() {
                        parts_by_gear
                            .entry((line_num, span))
                            .or_insert(Vec::new())
                            .push(part.1);
                    }
                }
                symbols.push(span);
                continue;
            }

            if parser.match_char_if(is_symbol).is_some() {
                continue;
            }

            if parser.peek_position() == line.len() {
                break;
            }

            panic!(
                "Unrecognized character at line: {}, column: {}, character: {}",
                line_num,
                parser.peek_position(),
                &line[parser.peek_position()..parser.peek_position() + 1]
            );
        }

        prev_line_parts = parts;
        prev_line_symbols = symbols;
    }

    parts_by_gear
        .into_iter()
        .map(|kvp| kvp.1)
        .fold(0, |acc, vec| {
            if vec.len() != 2 {
                acc
            } else {
                acc + vec[0] * vec[1]
            }
        })
}

fn is_symbol(ch: char) -> bool {
    match ch {
        '@' | '#' | '$' | '%' | '&' | '*' | '-' | '_' | '+' | '=' | '/' | '>' | '<' | ',' | '~'
        | '`' | '\'' | '"' => true,

        _ => false,
    }
}

fn max(first: usize, second: usize) -> usize {
    if first > second {
        first
    } else {
        second
    }
}

fn min(first: usize, second: usize) -> usize {
    if first < second {
        first
    } else {
        second
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn real_question() {
        assert_eq!(
            format!("{}", 544664),
            format!("{}", super::part1(include_str!("../data.txt")))
        );
        assert_eq!(
            format!("{}", 84495585),
            format!("{}", super::part2(include_str!("../data.txt")))
        );
    }

    #[test]
    fn example() {
        let str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

        assert_eq!(format!("{}", 4361), format!("{}", super::part1(str)))
    }

    #[test]
    fn example_part2() {
        let str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

        println!("{}", super::part2(str));
        assert_eq!(format!("{}", 467835), format!("{}", super::part2(str)));
    }

    #[test]
    fn part2_side_by_side() {
        let side_by_side = "120*80";
        assert_eq!(
            format!("{}", 120 * 80),
            format!("{}", super::part2(side_by_side))
        );
    }

    #[test]
    fn part2_above() {
        let above = "120\n*..\n80";
        assert_eq!(format!("{}", 120 * 80), format!("{}", super::part2(above)));
    }
}
