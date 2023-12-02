use std::{iter::Peekable, str::Chars};

pub struct Parser<'i> {
    chars_iter: Peekable<Chars<'i>>,
}

impl<'i> Parser<'i> {
    pub fn new(source: &'i str) -> Self {
        Self {
            chars_iter: source.chars().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars_iter.peek().copied()
    }

    pub fn match_str(&mut self, str: &str) -> bool {
        for char in str.chars() {
            if !self.match_char(char) {
                return false;
            }
        }
        return true;
    }

    pub fn match_char(&mut self, ch: char) -> bool {
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

    pub fn match_int(&mut self) -> Option<isize> {
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
