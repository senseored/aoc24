use std::fs;

fn main() {
    // --snip--
    // let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    let file_path = "input/day3.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");
    let mut sum: i32 = 0;

    let mut seq: Vec<String> = Vec::new();

    for numbers in contents.split("mul(") {
        // println!("{}", numbers);
        seq.push(numbers.to_string());
    }
    seq.remove(0);

    sum = calcsum(seq.clone());

    println!("sum1: {}", sum);

    let mut seq2: Vec<String> = Vec::new();

    let mut seq3: Vec<String> = Vec::new();
    let mut seq4: Vec<String> = Vec::new();

    for numbers in contents.split("do") {
        seq2.push(numbers.to_string());
        println!("{}", numbers);
    }
    for i in 0..seq2.len() {
        if i == 0 || seq2[i].starts_with("()") {
            println!("{}", seq2[i]);
            seq3.push(seq2[i].to_string());
        }
    }

    for i in 0..seq3.len() {
        for numbers in seq3[i].split("mul(") {
            // println!("{}", numbers);
            seq4.push(numbers.to_string());
        }
    }

    sum = calcsum(seq4);
    // for numbers in contents.split("do") {
    //     // println!("{}", numbers);
    //     // seq.push(numbers.to_string());
    // }

    println!("sum2: {}", sum);
}

fn calcsum(seq: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    let mut seq2: Vec<String> = Vec::new();
    for i in 0..seq.len() {
        for numbers in seq[i].split(")") {
            // println!("{}", numbers);
            seq2.push(numbers.to_string());
        }
    }
    let mut seq3: Vec<String> = Vec::new();
    for i in 0..seq2.len() {
        for (j, char) in seq2[i].chars().enumerate() {
            if j == 0 {
                if char.is_numeric() {
                    seq3.push(seq2[i].to_string());
                    // println!("{}", seq2[i]);
                }
            }
        }
    }

    // println!("{:?}", seq3);

    let mut seq4: Vec<String> = Vec::new();

    for i in 0..seq3.len() {
        let mut check = true;
        for char in seq3[i].chars() {
            if !char.is_numeric() && char != ',' {
                check = false;
            }
        }
        if check {
            // println!("{}", seq3[i]);
            seq4.push(seq3[i].to_string());
        }
    }

    for i in 0..seq4.len() {
        let mut num1: i32 = 0;
        let mut num2: i32 = 0;
        for (j, numbers) in seq4[i].split(",").enumerate() {
            if j == 0 {
                num1 = numbers.parse::<i32>().unwrap();
            } else if j == 1 {
                num2 = numbers.parse::<i32>().unwrap();
            }
        }
        sum += num1 * num2;
    }

    return sum;
}
