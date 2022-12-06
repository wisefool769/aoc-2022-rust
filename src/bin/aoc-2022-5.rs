struct Instruction {
    n: usize,
    src: usize,
    dest: usize
}

type Column = Vec<char>;
type Crates = Vec<Column>;
struct Input {
    crates: Crates,
    instructions: Vec<Instruction>,
}

fn print_crates(crates: &Crates) {
    let mut max = 0;
    for c in crates {
        if c.len() > max {
            max = c.len();
        }
    }
    for i in 0..max {
        for c in crates {
            if i < c.len() {
                print!("{} ", c[i]);
            } else {
                print!("  ");
            }
        }
        println!("");
    }
}

fn parse_crates(input: &str) -> Crates{
    let last_row = input.split("\n").last().unwrap();
    let last_char = last_row.replace(" ", "").chars().last().unwrap();
    let columns: usize = last_char.to_digit(10).unwrap().try_into().unwrap();
    println!("{}", columns);
    let mut crates = vec![Vec::<char>::new(); columns];
    for line in input.lines().rev().skip(1) {
        for i in 0..columns {
            let crate_idx = i*4 + 1;
            let c = line.chars().nth(crate_idx).unwrap();
            if c.is_alphabetic()  {
                crates[i].push(c);
            }
        }
    }
    return crates;
}


fn parse_instruction(line: &str) -> Instruction {
    let instr = line
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    assert_eq!(instr.len(), 3);
    return Instruction {
        n: instr[0],
        src: instr[1],
        dest: instr[2],
    }
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    return input.split("\n").map(parse_instruction).collect::<Vec<Instruction>>();
}

fn read_input() -> Input {
    dotenv::dotenv().ok();
    let id = std::env::var("INPUT_DIR").expect("INPUT_DIR is not set");
    let input = id + "/5.txt";
    let raw_input_data = std::fs::read_to_string(input).unwrap();
    let mut iter = raw_input_data.split("\n\n");
    return Input{
        crates: parse_crates(iter.next().unwrap()),
        instructions: parse_instructions(iter.next().unwrap()),
    }
}

fn apply_instruction(crates: Crates, instruction: &Instruction, preserve_order: bool) -> Crates {
    let mut new_crates = crates.clone();
    let src_idx = instruction.src - 1;
    let dest_idx = instruction.dest - 1;
    let mut src = crates[src_idx].clone();
    let mut dest = crates[dest_idx].clone();
    let tail = src.split_off(src.len() - instruction.n);
    let iter = tail.iter();
    if preserve_order {
        dest.extend(iter);
    } else {
        dest.extend(iter.rev());
    }
    new_crates[src_idx] = src;
    new_crates[dest_idx] = dest;
    return new_crates;
}

fn mover(input: &Input, preserve_order: bool) -> String {
    println!("Applying instructions (preserve_order: {})", preserve_order);
    let processed = input.instructions.iter()
        .fold(input.crates.clone(), |acc, instr| apply_instruction(acc, &instr, preserve_order));
    print_crates(&processed);
    let answer: String = "".to_owned();
    return processed.iter().fold(answer, 
        |acc, column| acc + &column.iter().last().unwrap().to_string());
}


fn main() {
    let input = read_input();
    print_crates(&input.crates);
    println!("");
    println!("Mover 9000: {}", mover(&input, false));
    println!("Mover 9001: {}", mover(&input, true));
}