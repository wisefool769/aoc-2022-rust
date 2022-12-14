use std::collections::VecDeque;
use aoc_2022::read_input;


// takes in old and returns a * old * old + b * old + c
#[derive(Debug)]
#[derive(Clone)]
struct Updater {
    a: u64,
    b: u64,
    c: u64
}

fn update(updater: &Updater, old: u64) -> u64 {
    return updater.a * old.clone() * old.clone() + updater.b * old + updater.c;
}

#[derive(Debug)]
#[derive(Clone)]
struct Monkey {
    idx: i32,
    items: VecDeque<u64>,
    updater: Updater,
    div: u64,
    tt: usize,
    tf: usize,
    n: u64
}

fn get_idx(line: &str) -> i32 {
    return line.replace("Monkey ", "").replace(":", "").parse::<i32>().unwrap();
}

fn get_items(line: &str) -> VecDeque<u64> {
    let mut items = VecDeque::<u64>::new();
    for p in line.replace("Starting items: ", "").split(", ") {
        let i = p.trim().parse::<u64>().unwrap();
        items.push_back(u64::from(i));
    }
    items
}

fn get_updater(line: &str) -> Updater {
    let fp = line.split(" = ").last().unwrap();
    let terms = fp.split(" + ");
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for t in terms {
        if t.contains("old * old") {
            let ar = t.replace("old * old", "").replace("*", "");
            let at = ar.trim();
            a = if at.is_empty() { 1 } else { at.parse::<u64>().unwrap()};
        } else if t.contains("old") {
            let br = t.replace("old", "").replace("*", "");
            let bt = br.trim();
            b = if bt.is_empty() { 1 } else { bt.parse::<u64>().unwrap()};

        } else {
            c = t.replace(" ", "").parse::<u64>().unwrap();
        }
    }
    Updater{ a: a, b: b, c: c }
}

fn get_last_num(line: &str) -> i32 {
    line.split(" ").last().unwrap().parse::<i32>().unwrap()
}

fn parse_monkey(m: &str) -> Monkey {
    let mut lines = m.split("\n");
    let idx = get_idx(lines.next().unwrap());
    let items = get_items(lines.next().unwrap());
    let u = get_updater(lines.next().unwrap());
    let div = get_last_num(lines.next().unwrap()) as u64;
    let tt = get_last_num(lines.next().unwrap()) as usize;
    let tf = get_last_num(lines.next().unwrap()) as usize;
    Monkey{ idx: idx, items: items, updater: u, div: div, tt: tt, tf: tf, n: 0}
}


fn do_round(monkeys: &mut Vec<Monkey>, relax: bool) {
    let div = monkeys.iter().fold(1, |acc, m| acc * m.div);
    for mi in 0..monkeys.len() {
        let mut items = monkeys[mi].items.clone();
        let mut n = 0;
        while !items.is_empty() {
            let item = items.pop_front().unwrap();
            let w = update(&monkeys[mi].updater, item);
            let relaxed = if relax { w / 3 } else { w % div};
            let ti = if relaxed.clone() % monkeys[mi].div == 0 {monkeys[mi].tt} else {monkeys[mi].tf};
            monkeys[ti].items.push_back(relaxed);
            n += 1;
        }
        monkeys[mi].items = VecDeque::<u64>::new();
        monkeys[mi].n += n;
    }
}

fn play(monkeys: &mut Vec<Monkey>, rounds: i32, relax: bool) -> u64 {
    for _ in 0..rounds {
        do_round(monkeys, relax);
    }
    monkeys.sort_by(|a, b| b.n.cmp(&a.n));
    let mut mi = monkeys.iter();
    let mb = mi.next().unwrap().n * mi.next().unwrap().n;
    mb
}



fn main() {
    let raw_input = read_input(11);
    let mut monkeys: Vec<Monkey> = raw_input.split("\n\n").map(parse_monkey).collect();
    let mut monkeys2 = monkeys.clone();
    println!("{}", raw_input);
    monkeys.iter().for_each(|m| println!("{:?}", m));

    let p1 = play(&mut monkeys, 20, true);
    println!("p1: {}", p1);

    let p2 = play(&mut monkeys2, 10000, false);
    for m in monkeys2.iter() {
        println!("{} {}", m.idx, m.items.len());

    }
    println!("p2: {}", p2);

}