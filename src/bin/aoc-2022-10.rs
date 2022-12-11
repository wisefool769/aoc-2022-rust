use aoc_2022::read_input;

struct State1  {
    cycle: i32,
    x: i32,
    pt1: i32,
}


fn process_instruction(state: &State1, line: &str) -> State1 {
    let mut acc = state.pt1;
    let mut cycle = state.cycle;
    let mut x = state.x;
    cycle += 1;

    if line  == "noop" {
        if cycle % 40 == 20 {
            acc += state.x * cycle;
        }
    } else {
        if cycle % 40 == 20 {
            acc += state.x * cycle;
        }
        cycle += 1;
        let v = line.split(" ").last().unwrap().parse::<i32>().unwrap();
        x += v;
    }

    return State1 { cycle: cycle, x: x, pt1: acc };
}

fn part1(raw_input: &str) -> i32 {
    let state = State1 { cycle: 0, x: 1, pt1: 0 };
    let state_pt1 = raw_input.split("\n").fold(state, |acc, line| process_instruction(&acc, line));
    return state_pt1.pt1;
}

struct State2 {
    cycle: i32,
    x: i32,
    output: String
}

fn advance(state: &mut State2){
    let x = state.x;
    let mut c = state.cycle;
    let h  = c % 40 - 1;
    let mut o = state.output.clone();
    if (x - h).abs() < 2 {
        o += "#";
    } else {
        o += ".";
    }
    c += 1;
    if c % 40  == 1{
        o += "\n";
    }

    state.cycle = c;
    state.output = o;
}

fn part2(raw_input: &str) -> String {
    let mut state = State2 { cycle: 1, x: 1, output: "".to_owned() };
    for line in raw_input.split("\n") {
        advance(&mut state);
        if line != "noop" {
            advance(&mut state);
            let v = line.split(" ").last().unwrap().parse::<i32>().unwrap();
            state.x += v;
        }
    }
    state.output
}

fn main() {
    let raw_input = read_input(10);
    println!("{}", part1(&raw_input));
    println!("{}", part2(&raw_input));
}