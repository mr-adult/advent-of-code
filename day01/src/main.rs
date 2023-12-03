fn main() {
    let file_content = include_str!("../data.txt");
    println!("Part 1: {}", part1(file_content));
    println!("Part 2: {}", part2(file_content));
}

fn part1(data: &str) -> usize {
    let result = data
        .lines()
        .map(|line| {
            let mut first_num = None;
            let mut last_num = None;

            let mut chars = line.chars();
            while let Some(char) = chars.next() {
                match char {
                    '0'..='9' => {
                        if let None = first_num {
                            first_num = Some(char);
                        }
                        last_num = Some(char);
                    }
                    _ => {}
                }
            }

            return first_num
                .expect("line to have at least 1 digit")
                .to_digit(10)
                .unwrap() as usize
                * 10
                + last_num
                    .expect("line to have at least 1 digit")
                    .to_digit(10)
                    .unwrap() as usize;
        })
        .fold(0, |mut acc, item| {
            acc += item;
            acc
        });

    result
}

fn part2(data: &str) -> usize {
    let match_at = |i: usize, str: &str| {
        if (i + str.len()) > data.len() {
            return false;
        }

        return &data[i..(i + str.len())].to_lowercase() == str;
    };

    let mut result: usize = 0;
    let mut first_num = None;
    let mut last_num = None;

    for (i, char) in data.char_indices() {
        let mut set_nums = |value: char| {
            if let None = first_num {
                first_num = Some(value);
            }
            last_num = Some(value);
        };

        match char {
            '0'..='9' => {
                set_nums(char);
            }
            'o' | 'O' => {
                if match_at(i, "one") {
                    set_nums('1');
                }
            }
            't' | 'T' => {
                if match_at(i, "two") {
                    set_nums('2');
                } else if match_at(i, "three") {
                    set_nums('3');
                }
            }
            'f' | 'F' => {
                if match_at(i, "four") {
                    set_nums('4');
                } else if match_at(i, "five") {
                    set_nums('5');
                }
            }
            's' | 'S' => {
                if match_at(i, "six") {
                    set_nums('6');
                } else if match_at(i, "seven") {
                    set_nums('7');
                }
            }
            'e' | 'E' => {
                if match_at(i, "eight") {
                    set_nums('8');
                }
            }
            'n' | 'N' => {
                if match_at(i, "nine") {
                    set_nums('9');
                }
            }
            '\n' => {
                result += first_num
                    .expect("line to have at least 1 digit")
                    .to_digit(10)
                    .unwrap() as usize
                    * 10
                    + last_num
                        .expect("line to have at least 1 digit")
                        .to_digit(10)
                        .unwrap() as usize;

                first_num = None;
                last_num = None;
            }
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn cases() {
        assert_eq!(
            format!("{}", 54953),
            format!("{}", super::part1(include_str!("../data.txt")))
        );
        assert_eq!(
            format!("{}", 53868),
            format!("{}", super::part2(include_str!("../data.txt")))
        );
    }
}
