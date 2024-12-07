use std::fs;

fn main() {
    // let file_path = "input/test.txt";
    let file_path = "input/day7.txt";
    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut correct: Vec<bool> = Vec::new();
    let mut correct_p2: Vec<bool> = Vec::new();
    let mut sum = 0;
    let mut targets = Vec::new();
    for (i, lines) in contents.lines().enumerate() {
        correct.push(false);
        correct_p2.push(false);
        let mut line = lines.split(':');
        let target: i64 = line.next().map(|s| s.parse().unwrap()).unwrap();
        targets.push(target);
        let numbers: Vec<i64> = line
            .next()
            .map(|ss| ss.split_whitespace().map(|s| s.parse::<i64>().unwrap()))
            .unwrap()
            .collect();
        correct[i] = calculate(target, &numbers);
        correct_p2[i] = calculate_p2(target, &numbers);
    }
    for i in 0..correct.len() {
        if correct[i] {
            sum += targets[i];
        }
    }
    println!("part 1: {}", sum);
    sum = 0;
    for i in 0..correct_p2.len() {
        if correct_p2[i] {
            sum += targets[i];
        }
    }
    println!("part 2: {}", sum);
}

fn calculate(target: i64, numbers: &[i64]) -> bool {
    fn recursive(target: i64, current: i64, rest: &[i64]) -> bool {
        if rest.is_empty() {
            return current == target;
        }
        if target < current {
            return false;
        }
        if recursive(target, current * rest[0], &rest[1..]) {
            return true;
        }
        if recursive(target, current + rest[0], &rest[1..]) {
            return true;
        }
        false
    }
    recursive(target, numbers[0] as i64, &numbers[1..] as &[i64])
}

fn calculate_p2(target: i64, numbers: &[i64]) -> bool {
    fn recursive(target: i64, current: i64, rest: &[i64]) -> bool {
        if rest.is_empty() {
            return current == target;
        }
        if target < current {
            return false;
        }
        if recursive(target, current * rest[0], &rest[1..]) {
            return true;
        }
        if recursive(target, current + rest[0], &rest[1..]) {
            return true;
        }
        if recursive(
            target,
            format!("{:?}{:?}", current, rest[0]).parse().unwrap(),
            &rest[1..],
        ) {
            return true;
        }
        false
    }
    recursive(target, numbers[0] as i64, &numbers[1..] as &[i64])
}
