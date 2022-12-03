use std::fs;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let buff_reader = BufReader::new(fs::File::open("/Users/aashiq/projects/aoc-2022/inputs/1.txt")?);
    let mut elves: Vec<i32> = Vec::new();
    let mut elf_sum: i32 = 0;
    for line in buff_reader.lines() {
        let line = line.expect("bad line");
        if line.is_empty() {
            elves.push(elf_sum);
            elf_sum = 0;
        } else {
            let weight: i32 = line.parse().expect("could not parse");
            elf_sum += weight;
        }
    }
    elves.push(elf_sum);
    elves.sort();
    println!("max: {}", elves.iter().last().unwrap());
    println!("top 3: {}", elves.iter().rev().take(3).sum::<i32>());
    Ok(())
}
