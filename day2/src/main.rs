use std::fs;

fn main() {
    let file_path = "input/day2.txt";
    // let file_path = "input/test.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum: i32 = 0;
    println!("total lines: {}", contents.lines().count());
    println!("{}", contents);

    for line in contents.lines() {
        // println!("{}", line);
        let mut incdec: bool = false;
        let mut safe: bool = true;
        let mut seq: Vec<i32> = Vec::new();
        for number in line.split_whitespace() {
            seq.push(number.parse().unwrap());
        }
        for (i, _number) in seq.iter().enumerate() {
            let difference: i32;
            if i > 0 {
                difference = seq[i] - seq[i - 1];
                if i == 1 {
                    incdec = difference > 0;
                }
                if incdec && (difference <= 0 || difference >= 4) {
                    safe = false;
                }
                if !incdec && (difference >= 0 || difference <= -4) {
                    safe = false;
                }
            }
        }
        // println!("{:?}", seq);
        // println!("{} - {}", line, safe);
        if safe {
            sum = sum + 1;
        }
    }

    println!("safe test 1: {}", sum);

    sum = 0;

    for line in contents.lines() {
        let mut seq: Vec<i32> = Vec::new();
        let mut safe = false;
        for number in line.split_whitespace() {
            seq.push(number.parse().unwrap());
        }
        for (i, _number) in seq.iter().enumerate() {
            let mut seq2 = seq.clone();
            seq2.remove(i);
            if is_safe(seq.clone()) {
                safe = true;
            } else if is_safe(seq2.clone()) {
                safe = true;
            }
        }
        if safe {
            sum = sum + 1;
        }
    }

    println!("safe test 2: {}", sum);
}

fn is_safe(seq: Vec<i32>) -> bool {
    let mut incdec: bool = false;
    let mut safe: bool = true;
    for (i, _number) in seq.iter().enumerate() {
        let difference: i32;
        if i > 0 {
            difference = seq[i] - seq[i - 1];
            if i == 1 {
                incdec = difference > 0;
            }
            if incdec && (difference <= 0 || difference >= 4) {
                safe = false;
            }
            if !incdec && (difference >= 0 || difference <= -4) {
                safe = false;
            }
        }
    }
    safe
}
