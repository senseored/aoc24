use std::fs;

fn main() {
    let file_path = "input/day2.txt";
    // let file_path = "input/test.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut sum: i32 = 0;
    println!("total lines: {}", contents.lines().count());
    println!("{}", contents);

    contents.lines().for_each(|line| {
        let (mut incdec, mut safe) = (false, true);
        let mut seq: Vec<i32> = Vec::new();
        line.split_whitespace().for_each(|number| {
            seq.push(number.parse().unwrap());
        });
        seq.iter().enumerate().for_each(|(i, _)| {
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
        });
        if safe {
            sum += 1;
        }
    });

    println!("safe test 1: {}", sum);

    sum = 0;

    contents.lines().for_each(|line| {
        let mut seq: Vec<i32> = Vec::new();
        let mut safe = false;
        line.split_whitespace().for_each(|number| {
            seq.push(number.parse().unwrap());
        });
        seq.iter().enumerate().for_each(|(i, _)| {
            let mut seq2 = seq.clone();
            seq2.remove(i);
            if is_safe(seq.clone()) || is_safe(seq2.clone()) {
                safe = true;
            }
        });
        if safe {
            sum += 1;
        }
    });

    println!("safe test 2: {}", sum);
}

fn is_safe(seq: Vec<i32>) -> bool {
    let (mut incdec, mut safe) = (false, true);
    seq.iter().enumerate().for_each(|(i, _)| {
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
    });
    safe
}
