use std::fs;
use std::io::{BufRead, BufReader, Error};
use itertools::Itertools;
use std::env;
use dotenv::dotenv;

fn main() -> Result<(), Error> {
    dotenv().ok();
    let input_dir = env::var("INPUT_DIR").expect("INPUT_DIR is not set");
    let input = input_dir + "/1.txt";
    let elves: Vec<i32> = itertools::sorted(BufReader::new(fs::File::open(input)?)
        .lines().map(|l| l.unwrap())
        .group_by(|l| !l.is_empty()).into_iter()
        .map(|(_, group)| {
            group.map(|g| g.parse::<i32>().unwrap_or(0)).sum::<i32>()
        })).collect();
    println!("max: {}", elves.iter().last().unwrap());
    println!("top 3: {}", elves.iter().rev().take(3).sum::<i32>());
    Ok(())
}
