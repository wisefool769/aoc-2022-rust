use aoc_2022::read_input;

struct Tree {
    row_idx: usize,
    col_idx: usize,
    height: u8
}

struct Forest {
    grid: Vec<Vec<u8>>,
    nrows: usize,
    ncols: usize
}

fn is_visible(forest: &Forest, tree: &Tree) -> bool{
    let rn = forest.nrows;
    let cn = forest.ncols;
    let ri = tree.row_idx;
    let ci = tree.col_idx;
    let v = tree.height;

    // left
    if !(0..ci).any(|i| forest.grid[ri][i] >= v) {
        return true;
    }
    // right
    if !(ci + 1..cn).any(|i| forest.grid[ri][i] >= v) {
        return true;
    }
    // top
    if !(0..ri).any(|i| forest.grid[i][ci] >= v) {
        return true;
    }
    // bottom
    if !(ri+1..rn).any(|i| forest.grid[i][ci] >= v) {
        return true;
    }

    false
}

fn count_visible(forest: &Forest) -> u32 {
    let mut s = 0;
    for row_idx in 0..forest.nrows {
        for col_idx in 0..forest.ncols {
            let height = forest.grid[row_idx][col_idx];
            let tree = Tree { row_idx: row_idx, col_idx: col_idx, height: height };
            if is_visible(&forest, &tree) {
                s += 1;
            }
        }
    }
    s
}

fn scenic_score(forest: &Forest, tree: &Tree) -> u32 {
    let v = tree.height;
    let ri = tree.row_idx;
    let ci = tree.col_idx;
    let rn = forest.nrows;
    let cn = forest.ncols;

    // left
    let mut lc = 0;
    for i in 0..ci {
        lc += 1;
        if forest.grid[ri][ci-i-1] >= v {
            break;
        }
    }
    // right
    let mut rc = 0;
    for i in ci+1..cn {
        rc += 1;
        if forest.grid[ri][i] >= v {
            break;
        }
    }

    // top 
    let mut tc = 0;
    for i in 0..ri {
        tc += 1;
        if forest.grid[ri - i -1][ci] >= v {
            break;
        }
    }

    // bottom 
    let mut bc = 0;
    for i in ri+1..rn {
        bc += 1;
        if forest.grid[i][ci] >= v {
            break;
        }
    }
    lc * rc * tc * bc
}

fn max_scenic(forest: &Forest) -> u32 {
    let nr = forest.nrows;
    let nc = forest.ncols;
    let mut m = 0;
    for ri in 0..nr {
        for ci in 0..nc {
            let tree = Tree{ row_idx: ri, col_idx: ci, height: forest.grid[ri][ci]};
            m = std::cmp::max(m, scenic_score(&forest, &tree))
        }
    }
    m
}

fn parse_input(input: &str) -> Forest {
    let mut grid = Vec::<Vec<u8>>::new();
    for line in input.split("\n") {
        let mut tree_line = Vec::<u8>::new();
        for c in line.chars() {
            tree_line.push(c as u8);
        }
        grid.push(tree_line);
    }
    let nrows = grid.len();
    let ncols = grid[0].len();
    return Forest{ grid: grid, nrows: nrows, ncols: ncols};
}


fn main() {
    let raw_input = read_input(8);
    let forest = parse_input(&raw_input);
    let answer1 = count_visible(&forest);
    let answer2 = max_scenic(&forest);
    println!("answer1: {}", answer1);
    println!("answer2: {}", answer2);
}