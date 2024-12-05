use std::env;
use std::fs;

fn main() {
    // --snip--
    let file_path = "input/day1.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("With text:\n{contents}");

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        for (i, number) in line.split_whitespace().enumerate() {
            if i % 2 == 0 {
                list1.push(number.parse().unwrap());
            } else {
                list2.push(number.parse().unwrap());
            }
        }
    }
    list1.sort();
    list2.sort();

    let mut result: Vec<i32> = Vec::new();

    for i in 0..list1.len() {
        if list1[i] > list2[i] {
            result.push(list1[i] - list2[i]);
        } else {
            result.push(list2[i] - list1[i]);
        }
    }

    let mut sum: i32 = 0;
    for i in 0..result.len() {
        sum += result[i];
    }
    println!("part 1: {sum:?}");

    let mut list3: Vec<i32> = Vec::new();

    for i in 0..list1.len() {
        let mut occurances: i32 = 0;
        for j in 0..list2.len() {
            if list2[j] == list1[i] {
                occurances += 1;
            }
        }
        list3.push(occurances * list1[i]);
    }
    let mut sum2: i32 = 0;
    for i in 0..list3.len() {
        sum2 += list3[i];
    }

    println!("part 2: {sum2:?}");
}
