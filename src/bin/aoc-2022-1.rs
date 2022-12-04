use std::env;
use dotenv::dotenv;

fn total(s: &str) -> i32 {
    return s.split("\n").map(|x| x.parse::<i32>().unwrap()).sum::<i32>();
}

fn main() {
    dotenv().ok();
    let input_dir = env::var("INPUT_DIR").expect("INPUT_DIR is not set");
    let input = input_dir + "/1.txt";
    let raw_input_data = std::fs::read_to_string(input).unwrap();
    let mut elves = raw_input_data.split("\n\n").map(total).collect::<Vec<i32> >();
    elves.sort_by(|a, b| b.cmp(a));
    println!("max: {}", elves[0]);
    println!("top 3: {}", elves.iter().take(3).sum::<i32>());
}
