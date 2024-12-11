use std::collections::HashMap;
use std::fs;

fn main() {
    // let file_path = "input/test.txt";
    let file_path = "input/day11.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let blinks = 75;

    println!("{contents}");
    let stones: Vec<u64> = contents
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut lookup = HashMap::with_capacity(140_000);

    let count = count_stones(&stones, blinks, &mut lookup);
    println!("count: {count}");
}

fn count_stones(stones: &[u64], blinks: u64, lookup: &mut HashMap<(u64, u64), u64>) -> u64 {
    stones
        .iter()
        .fold(0, |i, stone| i + count_stone(*stone, blinks, lookup))
}

fn count_stone(stone: u64, blinks: u64, lookup: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }

    let key = (stone, blinks);

    if let Some(count) = lookup.get(&key) {
        return *count;
    }

    let (left, right) = next_stone(stone);
    let next_gen = blinks - 1;

    let mut count = count_stone(left, next_gen, lookup);
    if let Some(right) = right {
        count += count_stone(right, next_gen, lookup);
    }

    lookup.insert(key, count);

    count
}

fn next_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }

    if stone.checked_ilog10().unwrap() % 2 != 0 {
        return split_in_two(stone);
    }

    (stone * 2024, None)
}

fn split_in_two(stone: u64) -> (u64, Option<u64>) {
    let stone_str = stone.to_string();
    let (left, right) = stone_str.split_at(stone_str.len() / 2);

    (left.parse().unwrap(), Some(right.parse().unwrap()))
}
