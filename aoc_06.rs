#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! itertools = "0.10"
//! ```

use itertools::Itertools;

fn main() {

    let input = std::fs::read_to_string("aoc").expect(r#"Could not find Advent of Code input (should be a sibling file called "aoc")"#);

    let part_1 = input.as_bytes().windows(4).enumerate()
        .find(|(_, pack)| pack.iter().all_unique())
        .map(|(ix, _)| ix + 4)
        .unwrap();

    println!("{part_1}");

    let part_2 = input.as_bytes().windows(14).enumerate()
        .find(|(_, pack)| pack.iter().all_unique())
        .map(|(ix, _)| ix + 14)
        .unwrap();

    println!("{part_2}");
}
