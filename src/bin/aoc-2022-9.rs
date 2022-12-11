use aoc_2022::read_input;
type Pos = (i32, i32);

#[derive(Debug)]
enum Direction {
    R = 0,
    U = 1,
    L = 2,
    D = 3,
}

struct Instruction {
    direction: Direction,
    distance: u32,
}
impl Instruction {
    fn print(&self) {
        println!("{:?} {}", self.direction, self.distance);
    }
}

fn parse_direction(c: char) -> Direction {
    match c {
        'R' => Direction::R,
        'U' => Direction::U,
        'L' => Direction::L,
        'D' => Direction::D,
        _ => panic!("Invalid direction"),
    }
}

fn line_to_instruction(line: &str) -> Instruction {
    let mut parts = line.split(" ");
    let dp = parts.next().unwrap();
    let dc = dp.chars().next().unwrap();
    let direction = parse_direction(dc);
    let distance = parts.next().unwrap().parse::<u32>().unwrap();
    Instruction { direction: direction, distance: distance }
}

fn direction_to_vec(d: &Direction) -> Pos {
    match d {
        Direction::R => (1, 0),
        Direction::U => (0, 1),
        Direction::L => (-1, 0),
        Direction::D => (0, -1),
    }
}

fn move_one(x1: i32, x2: i32) -> i32{
    if x2 ==  x1 {
        return x1;
    }
    x1 + (x2 - x1) / (x2 - x1).abs()
}

fn get_rope_move(l1: &Pos, l2: &Pos) -> Pos{
    let diff = (l1.0 - l2.0, l1.1 - l2.1);
    let max_diff = std::cmp::max(diff.0.abs(), diff.1.abs());
    let must_move = max_diff > 1;
    if !must_move {
        return *l2;
    }

    (move_one(l2.0, l1.0),move_one(l2.1, l1.1))
}

fn get_num_visited(instructions: &Vec<Instruction>, rope_size: usize) -> usize {
    let mut visited: std::collections::HashSet<Pos> = std::collections::HashSet::new();
    let start_pos: Pos = (0, 0);
    let mut body = vec![start_pos; rope_size];
    let tail_pos = rope_size - 1;
    visited.insert(start_pos);
    for i in instructions {
        // i.print();
        let vec = direction_to_vec(&i.direction);
        for _ in 0..i.distance {
            body[0] = (body[0].0 + vec.0, body[0].1 + vec.1);
            for j in 1..rope_size {
                body[j] = get_rope_move(&body[j-1], &body[j]);
            }
            // println!("  head: {:?} tail: {:?}", body[0], body[tail_pos]);
            visited.insert(body[tail_pos]);
        }
    }
    visited.len()
}

fn main() {
    let raw_input = read_input(9);
    let instructions = raw_input.lines().map(line_to_instruction).collect::<Vec<Instruction>>();
    for i in &instructions {
        i.print();
    }
    println!("pt1: {}", get_num_visited(&instructions, 2));
    println!("pt2: {}", get_num_visited(&instructions, 10));
}