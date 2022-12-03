use std::{io::BufRead};
use std::collections::HashSet;

// Lowercase item types a through z have priorities 1 through 26.
// Uppercase item types A through Z have priorities 27 through 52.
fn get_priority(c: char) -> i32 {
    if c.is_ascii_lowercase() {
        return c as i32 - 'a' as i32 + 1;
    } else if c.is_ascii_uppercase() {
        return c as i32 - 'A' as i32 + 27;
    } else {
        panic!("invalid item type");
    }
}

fn get_dupe_prio(elf: &String) -> i32 {
    let n = elf.len();
    let seen: HashSet<char> = HashSet::from_iter(elf[..n/2].chars());
    let duped = elf[n/2..].chars().find(|c| seen.contains(c)).unwrap();
    return get_priority(duped);
}

fn elf_group_badge_prio(group: &[String]) -> i32 {
    assert_eq!(group.len(), 3);
    let set_iter = group.iter()
        .map(|elf| {elf.chars().collect::<HashSet<char> >()});
    let first = set_iter.clone().next().unwrap().clone();
    let intersection = set_iter.fold(first, |acc, s| acc.intersection(&s).cloned().collect());
    assert_eq!(intersection.len(), 1);
    let badge = intersection.iter().next().unwrap();
    return get_priority(*badge);
}

fn read_input() -> Vec<String> {
    let input_dir = std::env::var("INPUT_DIR").expect("INPUT_DIR is not set");
    let input = input_dir + "/3.txt";
    let file = std::fs::File::open(input).expect("failed to open");
    return std::io::BufReader::new(file).lines().collect::<Result<_, _>>().unwrap();
}

fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();
    let elves = read_input();
    let total: i32 = elves.iter().map(get_dupe_prio).sum();
    let badge_total: i32 = elves.chunks(3).map(elf_group_badge_prio).sum();
    println!("total: {}", total);
    println!("badge_total: {}", badge_total);
    Ok(())
}
