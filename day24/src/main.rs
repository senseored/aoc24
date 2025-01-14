use std::fs;

fn main() {
    let contents = fs::read_to_string("input/test.txt").unwrap();
    // let contents = fs::read_to_string("input/day23.txt").unwrap();

    println!("{}", contents);
}
