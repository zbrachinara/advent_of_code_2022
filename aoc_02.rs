#!/usr/bin/env rust-script

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Move { Rock=0, Paper=1, Scissors=2 }

#[repr(u8)]
#[derive(Debug)]
enum Outcome { Win=6, Lose=0, Tie=3 }

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            c => panic!("incorrect character: {}", c),
        }
    }
} 

impl From<u8> for Move {
    fn from(u: u8) -> Self {
        unsafe { std::mem::transmute(u % 3) }
    }
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Tie,
            'Z' => Self::Win,
            c => panic!("incorrect character: {}", c),
        }
    }
}

impl Move {
    fn wins_over(self, other: Self) -> Outcome {
        if self == other {
            Outcome::Tie
        } else if (self as u8 + 1) % 3 == (other as u8) {
            Outcome::Lose
        } else {
            Outcome::Win
        }
    }

    fn score(self, match_with: Self) -> u32 {
        (self as u8 + 1 + self.wins_over(match_with) as u8) as u32
    }

    fn for_outcome(self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => Self::from(self as u8 + 1),
            Outcome::Lose => Self::from(self as u8 + 2),
            Outcome::Tie => self
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("aoc").expect(r#"Could not find Advent of Code input (should be a sibling file called "aoc")"#);

    let out = input.split("\n")
        .flat_map(|s| {
            let mut ch = s.chars();
            let first = ch.next()?;
            let third = ch.nth(1)?;
            Some((Move::from(first), Outcome::from(third)))
        })
        .inspect(|c| println!("{c:?}"))
        .map(|(a, b)| (a, a.for_outcome(b)))
        .inspect(|c| println!("{c:?}"))
        .map(|(a, b)| b.score(a))
        .inspect(|c| println!("{c:?}"))
        .sum::<u32>();

    println!("{out:?}");
}
