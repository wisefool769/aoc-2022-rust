use aoc_2022::read_input;
use std::collections::VecDeque;

struct Input {
    nr: usize,
    nc: usize,
    grid: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize)
}

fn parse_input(raw_input: &str) -> Input {
    let nc = raw_input.split("\n").next().unwrap().len();
    let nr = raw_input.split("\n").count();
    let mut grid = Vec::<Vec<u8>>::new();
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;

    for (row_idx, line) in raw_input.split("\n").enumerate() {
        let mut grid_line = Vec::<u8>::new();
        for (col_idx, c) in line.chars().enumerate() {
            if c == 'E' {
                grid_line.push(26);
                end = Some((row_idx, col_idx));
            } else if c == 'S' {
                grid_line.push(1);
                start = Some((row_idx, col_idx));
            } else {
                grid_line.push(c as u8 - 97 + 1);
            }
        }
        grid.push(grid_line);
    }
    Input{ nr: nr, nc: nc, grid: grid, start: start.unwrap(), end: end.unwrap()}
}

fn get_neighbors(input: &Input, cell: &(usize, usize)) -> Vec<(usize, usize)>{
    let nr = input.nr;
    let nc = input.nc;
    let mut neighbors = Vec::<(usize, usize)>::new();
    if cell.0 > 0 {
        neighbors.push((cell.0 - 1, cell.1));
    }
    if cell.0 < nr - 1 {
        neighbors.push((cell.0 + 1, cell.1));
    }
    if cell.1 > 0 {
        neighbors.push((cell.0, cell.1 - 1));
    }
    if cell.1 < nc - 1 {
        neighbors.push((cell.0, cell.1 + 1));
    }
    neighbors
}

struct Answer { 
    part1: u32,
    part2: u32
}

fn traverse(input: &Input) -> Answer {
    let nr = input.nr;
    let nc = input.nc;
    let grid = &input.grid;
    let start = input.start;
    let end = input.end;
    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(end);
    let mut trip: Vec<Vec<Option<u32>>> = vec![vec![None; nc]; nr];
    trip[end.0][end.1] = Some(0);
    // results for all cells with lowest elevation
    let mut astep = Vec::<u32>::new();
    let mut part1: Option<u32> = None;

    while !queue.is_empty() {
        println!("queue: {:?}", queue.len());
        let pos = queue.pop_front().unwrap();
        let n = trip[pos.0][pos.1].unwrap();
        if pos == start {
            part1 = Some(n);
            astep.push(n);
        } else {
            let h = grid[pos.0][pos.1];
            for neighbor in get_neighbors(&input, &pos){
                // don't visit things more than once
                if trip[neighbor.0][neighbor.1] != None {
                    continue;
                }
                // don't visit from neighbors that are too short
                let nh = grid[neighbor.0][neighbor.1];
                if nh + 1 < h {
                    continue;
                }
                trip[neighbor.0][neighbor.1] = Some(n+1);

                // push back so we always visit the shortest path first
                queue.push_back(neighbor);
            }

            if h == 1 {
                astep.push(n);
            }
        }
    }
    astep.sort();
    Answer { part1: part1.unwrap(), part2: astep[0] }
}

fn main() {
    let raw_input = read_input(12);
    let input = parse_input(&raw_input);
    let answer = traverse(&input);
    println!("Part 1: {}", answer.part1);
    println!("Part 2: {}", answer.part2);

}