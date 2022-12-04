#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! itertools = "0.10"
//! ```

use itertools::Itertools;

fn priority(ch: u8) -> u8 {
    match ch {
        b'a'..=b'z' => ch - b'a' + 1,
        b'A'..=b'Z' => ch - b'A' + 27,
        _ => panic!("Invalid input format (compartments should only contain a..z or A..Z)")
    }
}

fn main() {
    
    let input = std::fs::read_to_string("aoc").expect(r#"Could not find Advent of Code input (should be a sibling file called "aoc")"#);

    let part_1 = input.split("\n")
        .map(|line| {
            let line = line.as_bytes();
            line.split_at(line.len() / 2)      
        })
        .filter_map(|(first, second)| {
            let set = first.iter().collect::<std::collections::HashSet<_>>();
            second.iter().find(|c| set.contains(c))
        })
        .map(|&ch| priority(ch) as u32)
        .sum::<u32>();

    println!("{part_1:?}");

    let part_2 = input.split("\n")
        .filter_map(|line| (line.len() > 0).then_some(line.as_bytes()))
        .chunks(3).into_iter()
        .filter_map(|group| {
            let (a, b, c) = group.collect_tuple().unwrap();
            let mut set = a.iter().collect::<std::collections::HashSet<_>>();

            set.retain(|u| b.contains(u));
            c.iter().find(|it| set.contains(it))
        })
        .map(|&ch| priority(ch) as u32)
        .sum::<u32>() ;

    println!("{part_2:?}");

}
