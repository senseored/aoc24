use std::env;
use std::fs;

fn main() {
    // --snip--
    let file_path = "input/test.txt";
    // let file_path = "input/day3.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");
    let mut sum: i32 = 0;
    for (i, char) in contents.chars().enumerate() {
        if char == 'm' {
            sum += 1;
        }
    }
    for numbers in contents.split(',') {
        println!("{}", numbers);
    }
    println!("sum: {}", sum);
}

