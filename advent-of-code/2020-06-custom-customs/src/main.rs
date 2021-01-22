use std::fs;

const INPUT_FILE: &str = "./input.txt";


fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap_or_else(|e| panic!("Could not read file at {}: {}", INPUT_FILE, e));
    let groups: Vec<&str> = input.split("\n\n").collect();

    println!("{:?}", groups);
}
