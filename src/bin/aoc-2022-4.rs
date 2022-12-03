type Range = (i32, i32);

fn parse_part(part: &str) -> Range {
    let split: Vec<&str> = part.split("-").collect();
    return (split[0].parse::<i32>().unwrap(),split[1].parse::<i32>().unwrap());
}
fn parse_line(line: &str) -> (Range, Range) {
    println!("{}", line);
    let parts: Vec<&str> = line.split(",").collect();
    return (parse_part(parts[0]), parse_part(parts[1]));
}

fn rc(r1: Range, r2: Range) -> bool {
    return r2.0 >= r1.0 && r2.1 <= r1.1;
}

fn range_contains(r1: Range, r2: Range) -> bool {
    return rc(r1, r2) || rc(r2, r1);
}

fn range_overlap(r1: Range, r2: Range) -> bool {
    return (r1.1 >= r2.0) && (r1.0 <= r2.1);
}

fn main() {
    dotenv::dotenv().ok();
    let input_dir = std::env::var("INPUT_DIR").expect("INPUT_DIR is not set");
    let input = input_dir + "/4.txt";
    let raw_input_data = std::fs::read_to_string(input).unwrap();
    let assignments = raw_input_data.split("\n").map(parse_line);
    let total_contains = assignments.clone().filter(|(r1, r2)| range_contains(*r1, *r2)).count();
    let total_overlap = assignments.filter(|(r1, r2)| range_overlap(*r1, *r2)).count();
    println!("total_contains: {:?}", total_contains);
    println!("total_overlap: {:?}", total_overlap);
}