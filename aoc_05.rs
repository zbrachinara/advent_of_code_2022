#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! itertools = "0.10"
//! ```

use itertools::Itertools;

fn get_both_mut<T>(slice: &mut [T], a: usize, b: usize) -> (&mut T, &mut T) {
    if a > b {
        let (slice_b, slice_a) = slice.split_at_mut(a);
        (&mut slice_a[0], &mut slice_b[b])
    } else if a < b {
        let (slice_a, slice_b) = slice.split_at_mut(b);
        (&mut slice_a[a], &mut slice_b[0])
    } else {
        panic!("indices for {a} and {b} should not be equal")
    }
}

fn main() {

    let input = std::fs::read_to_string("aoc").expect(r#"Could not find Advent of Code input (should be a sibling file called "aoc")"#);

    let (crates_str, instructions_str) = input.split_once("\n\n").unwrap();

    let stacks = crates_str.rsplit_once("\n").map(|(crate_list, _)| {
        let mut stacks = std::iter::repeat_with(|| Vec::new()).take(9).collect_vec();
        crate_list.rsplit("\n").for_each(|row| {
            row.chars().skip(1).step_by(4).enumerate().filter(|&(_, c)| c != ' ')
                .for_each(|(ix, item)| stacks[ix].push(item));
        });
        stacks
    }).unwrap();
    let instructions = instructions_str.split("\n").filter(|&s| s.len() != 0)
        .filter_map(|instr| {
            let (_, a, _, b, _, c) = instr.split(" ").collect_tuple()?;
            Some(( 
                str::parse::<u8>(a).unwrap(),
                str::parse::<u8>(b).unwrap() - 1,
                str::parse::<u8>(c).unwrap() - 1,
            ))
        });

    let mut stacks_part_1 = stacks.clone();
    for (mov, from, to) in instructions.clone() {
        let (from, to) = get_both_mut(&mut stacks_part_1, from as usize, to as usize);
        to.extend(from.drain((from.len() - mov as usize)..).rev());
    }
    let part_1 = stacks_part_1.iter().map(|c| c.last().unwrap()).join("");
    println!("{part_1}");

    let mut stacks_part_2 = stacks;
    for (mov, from, to) in instructions {
        let (from, to) = get_both_mut(&mut stacks_part_2, from as usize, to as usize);
        to.extend(from.drain((from.len() - mov as usize)..));
    }
    let part_2 = stacks_part_2.iter().map(|c| c.last().unwrap()).join("");
    println!("{part_2}");
    

}
