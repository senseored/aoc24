use std::fs;

fn main() {
    // let file_path = "input/test.txt";
    let file_path = "input/day7.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");
}
