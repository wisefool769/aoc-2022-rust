use std::fs;
use std::io::{BufRead, BufReader, Error};
extern crate num;
#[macro_use]
extern crate num_derive;

#[derive(FromPrimitive)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
enum Move {
    Rock = 0,
    Paper,
    Scissors
}

impl Move {
    fn score(&self) -> i32 {
        return (*self as i32) + 1;
    }
}

#[derive(Clone)]
#[derive(Copy)]
enum Outcome {
    Win = 6,
    Lose = 0,
    Draw = 3
}

impl Outcome {
    fn score(&self) -> i32 {
        return *self as i32;
    }
}

fn decrypt_move_1 (their_move: &str) -> Move {
    match their_move {
        "A" => return Move::Rock,
        "B" => return Move::Paper,
        "C" => return Move::Scissors,
        "X" => return Move::Rock,
        "Y" => return Move::Paper,
        "Z" => return Move::Scissors,
        _ => panic!()
    }
}

fn decrypt_outcome(outcome: &str) -> Outcome {
    match outcome {
        "X" => return Outcome::Lose,
        "Y" => return Outcome::Draw,
        "Z" => return Outcome::Win,
        _ => panic!()
    }
}

fn get_outcome(my_move: Move, their_move: Move) -> Outcome {
    if my_move == their_move {
        return Outcome::Draw;
    }
    if(my_move as i32) == (((their_move as i32) + 1) % 3) {
        return Outcome::Win;
    }
    return Outcome::Lose;
}

fn get_total_score_1(line: &str) -> i32 {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let their_move = decrypt_move_1(parts[0]);
    let my_move = decrypt_move_1(parts[1]);
    let outcome = get_outcome(my_move, their_move);
    return my_move.score() + outcome.score();
}

fn fix_game(outcome: Outcome, their_move: Move) -> Move {
    match outcome {
        Outcome::Win => return num::FromPrimitive::from_i32(((their_move as i32) + 1) % 3).unwrap(),
        Outcome::Lose => return num::FromPrimitive::from_i32(((their_move as i32) + 2) % 3).unwrap(),
        Outcome::Draw => return their_move,
    }
}

fn get_total_score_2(line: &str) -> i32 {
    let parts: Vec<&str> = line.split_whitespace().collect();
    let their_move = decrypt_move_1(parts[0]);
    let outcome = decrypt_outcome(parts[1]);
    let my_move = fix_game(outcome, their_move);
    return my_move.score() + outcome.score();
}

fn main() -> Result<(), Error> {
    let buff_reader = BufReader::new(fs::File::open("/Users/aashiq/projects/aoc-2022/inputs/2.txt")?);
    let mut total_1 = 0;
    let mut total_2: i32 = 0;
    for line in buff_reader.lines() {
        let line = line.expect("bad line");
        total_1 += get_total_score_1(&line);
        total_2 += get_total_score_2(&line);
    }
    println!("total_1: {}", total_1);
    println!("total_2: {}", total_2);
    Ok(())
}
