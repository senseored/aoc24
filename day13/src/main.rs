use std::collections::HashMap;
use std::fs;

struct Machine {
    a: (u64, u64),
    b: (u64, u64),
    prize: (u64, u64),
}

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/day13.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");
    parse_input(contents);
}

fn parse_input(contents: String) {
    println!(
        "{:?}",
        contents.lines().filter(|l| l.starts_with("Button A: X+"))
    );
    println!(
        "{:?}",
        contents
            .lines()
            .filter(|l| l.strip_prefix("Button A: X+").is_some())
    );
}
