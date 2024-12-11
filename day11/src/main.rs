use std::sync::mpsc;
use std::{fs, thread};

// use std::fs;

fn main() {
    // let file_path = "input/test.txt";
    let file_path = "input/day11.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

    // let map = populate(&contents);
    let mut stones: Vec<u64> = Vec::new();
    contents.split_whitespace().for_each(|num| {
        stones.push(num.parse().unwrap());
    });
    let blinks = 75;

    // let mut count = stones.len().try_into().unwrap();

    let mut count = 0;
    println!("{:?}\ncount: {}", stones, count);
    // for i in 0..blinks {
    //     print!("Blink: {}, Stones: ", i + 1);
    //     // let mut new_stones: Vec<u64> = Vec::new();
    //     // for i in 0..stones.len() {
    //     //     // new_stones.append(&mut blink_thread(stones[i]));
    //     //     let stone = stones[i].clone();
    //     //     let handle = thread::spawn(move || blink_thread(stone));
    //     //     new_stones.append(&mut handle.join().unwrap()); //stones[i] = handle.join().unwrap()[0];
    //     // }
    //     // println!("{:?}", count_stones(&new_stones));
    //     // stones = new_stones;
    //     stones.iter().for_each(|&stone| {
    // stones.iter().for_each(|&stone| {
    //     count = recurse_count(stone, count, blinks);
    //     println!("from tree: stones: {:?}", count);
    // });
    println!("stones: {:?}", count);
    //         println!("i: {}, count: {}", i, count);
    //     });
    // stones = blink(stones);
    // let handle = thread::spawn(move || blink_thread(stone));
    // new_stones.append(&mut handle.join().unwrap());
    // }
    // println!("{:?}", count_stones(&stones));
    let (tx, rx) = mpsc::channel();

    let stones1 = stones.clone();
    let stones2 = stones.clone();
    let stones3 = stones.clone();
    let stones4 = stones.clone();
    let stones5 = stones.clone();
    let stones6 = stones.clone();
    let stones7 = stones.clone();
    let stones8 = stones.clone();
    let tx1 = tx.clone();
    thread::spawn(move || {
        tx1.send(recurse_count(stones1[0], 0, blinks)).unwrap();
    });
    let tx2 = tx.clone();
    thread::spawn(move || {
        tx2.send(recurse_count(stones2[1], 0, blinks)).unwrap();
    });
    let tx3 = tx.clone();
    thread::spawn(move || {
        tx3.send(recurse_count(stones3[2], 0, blinks)).unwrap();
    });
    let tx4 = tx.clone();
    thread::spawn(move || {
        tx4.send(recurse_count(stones4[3], 0, blinks)).unwrap();
    });
    let tx5 = tx.clone();
    thread::spawn(move || {
        tx5.send(recurse_count(stones5[4], 0, blinks)).unwrap();
    });
    let tx6 = tx.clone();
    thread::spawn(move || {
        tx6.send(recurse_count(stones6[5], 0, blinks)).unwrap();
    });
    let tx7 = tx.clone();
    thread::spawn(move || {
        tx7.send(recurse_count(stones7[6], 0, blinks)).unwrap();
    });
    let tx = tx.clone();
    thread::spawn(move || {
        tx.send(recurse_count(stones[7], 0, blinks)).unwrap();
    });

    let mut finalcount = 0;
    let mut x = 0;
    for recieved in rx {
        finalcount += recieved;
        x += 1;
        println!("Got: {}, finalcount: {}", recieved, finalcount);
        if x == 8 {
            break;
        }
    }
}
// fn count_stones(stones: &Vec<u64>) -> u64 {
//     stones.len().try_into().unwrap()
// }
//
// fn blink(stones: Vec<u64>) -> Vec<u64> {
//     let mut new_stones: Vec<u64> = Vec::new();
//     stones.iter().for_each(|&stone| {
//         new_stones.append(&mut blink_thread(stone));
//     });
//
//     println!("{:?}", count_stones(&new_stones));
//     new_stones
// }
//
// fn blink_thread(stone: u64) -> Vec<u64> {
//     let mut new_stones: Vec<u64> = Vec::new();
//     if stone == 0 {
//         new_stones.push(1);
//     } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
//         let stone_str = stone.to_string();
//         let left = &stone_str[0..stone_str.len() / 2];
//         let right = &stone_str[stone_str.len() / 2..stone_str.len()];
//         new_stones.push(left.parse().unwrap());
//         new_stones.push(right.parse().unwrap());
//     } else {
//         new_stones.push(stone * 2024);
//     }
//     // println!("{:?}", count_stones(&new_stones));
//
//     new_stones
// }
//
fn recurse_count(stone: u64, mut count: u64, blinks: u64) -> u64 {
    fn recursive(stone: u64, mut count: u64, mut blinks: u64) -> u64 {
        blinks -= 1;
        if blinks == 0 {
            count += 1;
            if count % 10000000 == 0 {
                println!("count: {}", count);
            }
            return count;
        }
        if stone == 0 {
            count = recursive(1, count, blinks);
        } else if (stone.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
            let stone_str = stone.to_string();
            let left = &stone_str[0..stone_str.len() / 2];
            let right = &stone_str[stone_str.len() / 2..stone_str.len()];
            count = recursive(left.parse().unwrap(), count, blinks);
            count = recursive(right.parse().unwrap(), count, blinks);
            // count += 1;
        } else {
            count = recursive(stone * 2024, count, blinks);
        }
        count
    }
    count = recursive(stone, count, blinks + 1);
    count
}
