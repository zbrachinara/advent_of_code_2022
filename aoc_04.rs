#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! itertools = "0.10"
//! ```

#[derive(Debug)]
struct Range{start: u16, end: u16}

impl Range {
    fn parse_pair(s: &str) -> (Self, Self) {
        let (a, b) = s.split_once(",").unwrap();
        (Self::parse_one(a), Self::parse_one(b))
    }

    fn parse_one(s: &str) -> Self {
        let (a, b) = s.split_once("-").unwrap();
        Self { start: str::parse(a).unwrap(), end: str::parse(b).unwrap() }
    }

    fn contained_within(&self, other: &Self) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn contained(a: &Self, b: &Self) -> bool {
        a.contained_within(b) || b.contained_within(a)
    }

    fn overlapped_by(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn overlap(a: &Self, b: &Self) -> bool {
        a.overlapped_by(b) || b.overlapped_by(a)
    }
}

fn main() {

    let input = std::fs::read_to_string("aoc").expect(r#"Could not find Advent of Code input (should be a sibling file called "aoc")"#);
    
    let part_1 = input.split("\n").filter(|line| line.len() != 0)
        .map(Range::parse_pair)
        .filter(|(a, b)| Range::contained(a, b))
        .count();

    println!("{part_1:?}");

    let part_2 = input.split("\n").filter(|line| line.len() != 0)
        .map(Range::parse_pair)
        .filter(|(a, b)| Range::overlap(a, b))
        .inspect(|x| println!("accepted: {x:?}"))
        .count();

    println!("{part_2:?}");
}
