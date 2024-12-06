use std::fs;

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    // let file_path = "input/day5.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

    let mut seq1: Vec<String> = Vec::new();
    let mut seq2: Vec<String> = Vec::new();
    let mut part2 = false;
    let mut rules: (Vec<Vec<i32>>, Vec<Vec<bool>>) = (Vec::new(), Vec::new());
    let mut pages: (Vec<Vec<i32>>, Vec<i32>) = (Vec::new(), Vec::new());

    for lines in contents.lines() {
        if lines.len() == 0 {
            part2 = true;
            continue;
        }
        if part2 {
            seq2.push(lines.to_string());
        } else {
            seq1.push(lines.to_string());
        }
    }
    for i in 0..seq1.len() {
        let mut rule: Vec<i32> = Vec::new();
        for part in seq1[i].split("|") {
            rule.push(part.parse().unwrap());
        }
        rules.0.push(rule);
        let mut result: Vec<bool> = Vec::new();
        result.push(false);
        result.push(false);
        rules.1.push(result);
    }
    for i in 0..seq2.len() {
        let mut page: Vec<i32> = Vec::new();
        for part in seq2[i].split(",") {
            page.push(part.parse().unwrap());
        }
        pages.0.push(page);
        pages.1.push(0);
    }
    println!("part 1: {:?}", seq1.clone());
    println!("part 2: {:?}", seq2.clone());
    // println!("rules: {:?}", rules.clone());
    // println!("pages: {:?}", pages.clone());

    let mut sum = 0;
    let mut sum2 = 0;
    let mut fixsum = 0;
    pages.1 = check_pages(rules.clone(), pages.clone());
    for i in 0..pages.1.len() {
        if pages.1[i] == -1 {
            sum += 1;
            println!("correct page: {:?}", pages.0[i]);
            sum2 += pages.0[i][pages.0[i].len() / 2];
        } else if pages.1[i] > 0 {
            fixsum += 1;
            println!("fix page: {:?}", pages.0[i]);
            let mut places: (usize, usize) = (0, 0);
            for j in 0..pages.0[i].len() {
                if rules.0[pages.1[i] as usize][1] == pages.0[i][j] {
                    places.0 = j;
                } else if rules.0[pages.1[i] as usize][0] == pages.0[i][j] {
                    places.1 = j;
                }
            }
            pages.0[i].swap(places.0, places.1);
            println!("fixed page: {:?}", pages.0[i]);
        }
    }
    println!("correct pages: {}", sum);
    println!("sum: {}", sum2);
    println!("pages to fix: {}", fixsum);
}

fn check_pages(
    rules: (Vec<Vec<i32>>, Vec<Vec<bool>>),
    mut pages: (Vec<Vec<i32>>, Vec<i32>),
) -> Vec<i32> {
    for i in 0..pages.0.len() {
        pages.1[i] = check_page(rules.clone(), (pages.0[i].clone(), pages.1[i].clone()));
    }
    pages.1
}

fn check_page(mut rules: (Vec<Vec<i32>>, Vec<Vec<bool>>), mut page: (Vec<i32>, i32)) -> i32 {
    for k in 0..rules.1.len() {
        rules.1[k][0] = false;
        rules.1[k][1] = false;
    }
    for j in 0..page.0.len() {
        for k in 0..rules.0.len() {
            if page.0[j] == rules.0[k][0] {
                if rules.1[k][1] {
                    page.1 = k as i32;
                    break;
                } else {
                    rules.1[k][0] = true;
                }
            } else if page.0[j] == rules.0[k][1] {
                rules.1[k][1] = true;
                if rules.1[k][0] {
                    if page.1 == 0 {
                        page.1 = -1;
                    }
                }
            }
        }
        // println!("rules: {:?}", rules.1.clone());
    }
    page.1
}
