pub fn read_input(n: u8) -> String {
    dotenv::dotenv().ok();
    let id = std::env::var("INPUT_DIR").expect("INPUT_DIR is not set");
    let day_string = n.to_string();
    let input = id + "/" + &day_string + ".txt";
    return std::fs::read_to_string(input).unwrap();
}