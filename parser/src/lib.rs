use core::num;
use std::{iter::Peekable, str::CharIndices};

pub struct Parser<'i> {
    source: &'i str,
    chars_iter: Peekable<CharIndices<'i>>,
}

impl<'i> Parser<'i> {
    pub fn new(source: &'i str) -> Self {
        Self {
            source,
            chars_iter: source.char_indices().peekable(),
        }
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars_iter.peek().into_iter().map(|opt| opt.1).next()
    }

    pub fn peek_position(&mut self) -> usize {
        match self.chars_iter.peek() {
            None => self.source.len(),
            Some((i, _)) => *i,
        }
    }

    pub fn match_str(&mut self, str: &str) -> Option<Span> {
        let start = self.peek_position();

        for char in str.chars() {
            if self.match_char(char).is_none() {
                return None;
            }
        }

        Span::new(start, self.peek_position()).as_opt()
    }

    pub fn match_char(&mut self, ch: char) -> Option<Span> {
        self.match_char_if(|source_ch| ch == source_ch)
    }

    pub fn match_char_if<P: FnMut(char) -> bool>(&mut self, mut predicate: P) -> Option<Span> {
        let start = self.peek_position();

        match self.chars_iter.peek() {
            None => {}
            Some((_, source_ch)) => {
                if predicate(*source_ch) {
                    self.chars_iter.next();
                }
            }
        }

        Span::new(start, self.peek_position()).as_opt()
    }

    pub fn match_char_while<P: FnMut(char) -> bool>(&mut self, mut predicate: P) -> Option<Span> {
        let start = self.peek_position();

        loop {
            if self.match_char_if(&mut predicate).is_none() {
                break;
            }
        }

        Span::new(start, self.peek_position()).as_opt()
    }

    pub fn match_uint(&mut self) -> Option<(Span, usize)> {
        let start = self.peek_position();

        let mut num_str = String::new();
        loop {
            match self.chars_iter.peek() {
                None => break,
                Some((_, ch)) => match ch {
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

        return Some((
            Span::new(start, self.peek_position()),
            num_str
                .parse::<usize>()
                .expect("string to be a valid integer at this point."),
        ));
    }

    pub fn match_int(&mut self) -> Option<(Span, isize)> {
        let start = self.peek_position();

        let is_negative = self.match_char('-').is_some();
        let mut numeric_value = self.match_uint()?.1 as isize;
        if is_negative {
            numeric_value = -numeric_value
        }

        return Some((Span::new(start, self.peek_position()), numeric_value));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    fn as_opt(self) -> Option<Self> {
        if self.start == self.end {
            None
        } else {
            Some(self)
        }
    }
}
