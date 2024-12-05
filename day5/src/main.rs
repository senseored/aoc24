use std::fs;

fn main() {
    // --snip--
    let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    // let file_path = "input/day5.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

    let mut seq1: Vec<String> = Vec::new();
    let mut seq2: Vec<String> = Vec::new();
    let mut num1: Vec<Vec<i32>> = Vec::new();
    let mut part2 = false;

    for lines in contents.lines() {
        if lines.len() == 0 {
            part2 = true;
            continue;
        }
        if part2 {
            seq2.push(lines.to_string());
        } else {
            seq1.push(lines.to_string());
        }
    }
    println!("part 1: {:?}", seq1.clone());
    println!("part 2: {:?}", seq2.clone());
}
