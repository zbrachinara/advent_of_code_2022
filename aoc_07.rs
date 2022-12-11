#!/usr/bin/env rust-script

//! ```cargo
//! [dependencies]
//! itertools = "0.10"
//! ```

use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
enum Shell<'a> {
    Enter {
        dirname: &'a str,
    },
    Exit,
    List,
    File {
        size: u32,
        name: &'a str,
    },
    Dir {
        dirname: &'a str,
    }
}

impl <'a> From<&'a str> for Shell<'a> {
    fn from(s: &'a str) -> Self {
        let by = s.as_bytes();
        if by[0] == b'$' {
            if let Some((_, _, dirname)) = s.splitn(3, " ").collect_tuple() {
                if dirname == ".." {
                    Self::Exit
                } else {
                    Self::Enter { dirname }
                }
            } else {
                Self::List
            }
        } else {
            let (size, name) = s.split_once(" ").unwrap();

            if size == "dir" {
                Self::Dir { dirname: name }
            } else {
                Self::File { size: str::parse(size).unwrap(), name}
            }
        }
    }
}

fn main() {

    let input = std::fs::read_to_string("aoc").expect(r#"Could not find Advent of Code input (should be a sibling file called "aoc")"#);

    let mut dirsizes = HashMap::new();
    let mut position = Vec::new();

    input.split("\n").filter(|s| s.len() > 0)
        .map(|cmd| Shell::from(cmd))
        .for_each(|cmd| {
            match cmd {
                Shell::Enter {dirname} => position.push(dirname),
                Shell::Exit => {position.pop();}
                Shell::File { size, .. } => {
                    let position = position.clone();
                    (1..=position.len()).rev().map(|i| &position[0..i])
                        .for_each(|entry| if let Some(mut size_target) = dirsizes.get_mut(entry) {
                            print!("increasing size of directory {entry:?} ");
                            *size_target += size;
                        } else {
                            print!("discovered directory {entry:?} and initialized size ");
                            dirsizes.insert(entry.to_vec(), size);
                        });
                }
                _ => ()
            };
            println!("with current position {position:?}");
        });

    let part_1 = dirsizes.values().filter(|&&size| size <= 100_000).inspect(|size| println!("{size}")).sum::<u32>();

    println!("{part_1:?}");

    const TOTAL_SIZE: u32 = 70_000_000;
    const NEED: u32 = 30_000_000;

    let free_space = dbg!(TOTAL_SIZE - dbg!(dirsizes[&["/"][..]]));
    let need_to_free = dbg!(NEED - free_space);

    let part_2 = dirsizes.values().filter(|&&size| size >= need_to_free).min();
    
    println!("{part_2:?}");

}
