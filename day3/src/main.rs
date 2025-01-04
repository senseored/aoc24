use std::fs;

fn main() {
    // --snip--
    // let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    let file_path = "input/day3.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut seq: Vec<String> = Vec::new();

    for numbers in contents.split("mul(") {
        // println!("{}", numbers);
        seq.push(numbers.to_string());
    }
    seq.remove(0);

    let mut sum = calcsum(seq.clone());

    println!("sum1: {}", sum);

    let mut seq2: Vec<String> = Vec::new();

    let mut seq3: Vec<String> = Vec::new();
    let mut seq4: Vec<String> = Vec::new();

    contents.split("do").for_each(|numbers| {
        seq2.push(numbers.to_string());
    });
    seq2.iter().enumerate().for_each(|(i, s)| {
        if i == 0 || s.starts_with("()") {
            seq3.push(s.to_string());
        }
    });
    seq3.iter().for_each(|s| {
        s.split("mul(").for_each(|n| {
            seq4.push(n.to_string());
        });
    });

    sum = calcsum(seq4);

    println!("sum2: {}", sum);
}

fn calcsum(seq: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    let mut seq2: Vec<String> = Vec::new();
    seq.iter().for_each(|s| {
        s.split(")").for_each(|n| {
            seq2.push(n.to_string());
        });
    });

    let mut seq3: Vec<String> = Vec::new();
    seq2.iter().for_each(|s| {
        s.chars().enumerate().for_each(|(i, c)| {
            if i == 0 && c.is_numeric() {
                seq3.push(s.to_string());
            }
        });
    });

    let mut seq4: Vec<String> = Vec::new();
    seq3.iter().for_each(|s| {
        let mut check = true;
        s.chars().for_each(|c| {
            if !c.is_numeric() && c != ',' {
                check = false;
            }
        });
        if check {
            seq4.push(s.to_string());
        }
    });

    seq4.iter().for_each(|s| {
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;
        s.split(",").for_each(|n| {
            if num1 == 0 {
                num1 = n.parse().unwrap();
            } else if num2 == 0 {
                num2 = n.parse().unwrap();
            }
        });
        sum += num1 * num2;
    });

    sum
}
