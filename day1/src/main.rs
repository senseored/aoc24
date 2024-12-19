use std::fs;

fn main() {
    // --snip--
    let file_path = "input/day1.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    contents.lines().for_each(|line| {
        line.split_whitespace().enumerate().for_each(|(i, number)| {
            if i % 2 == 0 {
                list1.push(number.parse().unwrap());
            } else {
                list2.push(number.parse().unwrap());
            }
        })
    });
    list1.sort();
    list2.sort();

    let mut result: Vec<i32> = Vec::new();

    list1.iter().zip(list2.iter()).for_each(|(a, b)| {
        if a > b {
            result.push(a - b);
        } else {
            result.push(b - a);
        }
    });

    println!("part 1: {:?}", result.iter().sum::<i32>());

    let mut part2 = 0;

    list1.iter().for_each(|a| {
        list2.iter().for_each(|b| {
            if a == b {
                part2 += a;
            }
        });
    });
    println!("part 2: {:?}", part2);
}
