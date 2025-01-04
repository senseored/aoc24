use std::fs;

fn main() {
    // let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    let file_path = "input/day5.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut seq1: Vec<String> = Vec::new();
    let mut seq2: Vec<String> = Vec::new();
    let mut part2 = false;
    let mut rules: (Vec<Vec<i32>>, Vec<Vec<bool>>) = (Vec::new(), Vec::new());
    let mut pages: (Vec<Vec<i32>>, Vec<i32>) = (Vec::new(), Vec::new());

    contents.lines().for_each(|line| {
        if part2 {
            seq2.push(line.to_string());
        } else if line.is_empty() {
            part2 = true;
        } else {
            seq1.push(line.to_string());
        }
    });
    seq1.iter().for_each(|s| {
        let mut rule: Vec<i32> = Vec::new();
        s.split("|").for_each(|p| {
            rule.push(p.parse().unwrap());
        });
        rules.0.push(rule);
        rules.1.push([false, false].to_vec());
    });
    seq2.iter().for_each(|s| {
        let mut page: Vec<i32> = Vec::new();
        s.split(",").for_each(|p| {
            page.push(p.parse().unwrap());
        });
        pages.0.push(page);
        pages.1.push(0);
    });
    println!("total rules: {}", rules.0.len());
    println!("total pages: {}", pages.0.len());
    println!("first page: {:?}", pages.0[0]);
    println!("last page: {:?}", pages.0[pages.0.len() - 1]);

    let mut sum = 0;
    let mut sum2 = 0;
    let mut fixsum = 0;
    let mut totfixsum = 0;
    pages.1 = check_pages(rules.clone(), pages.clone());
    for i in 0..pages.1.len() {
        if pages.1[i] == -1 {
            sum += 1;
            // println!("correct page: {:?}", pages.0[i]);
            sum2 += pages.0[i][pages.0[i].len() / 2];
        } else if pages.1[i] >= 0 {
            fixsum += 1;
            // println!("fix page: {:?}", pages.0[i]);
            // let mut j = 0;
            while pages.1[i] != -1 {
                pages.0[i] = fix_page(rules.clone(), (pages.0[i].clone(), pages.1[i]));
                pages.1[i] = check_page(rules.clone(), (pages.0[i].clone(), pages.1[i]));
                // j += 1;
                // println!("{}", pages.1[i]);
            }
        }
    }
    for i in 0..pages.1.len() {
        if pages.1[i] < 0 {
            // println!("fixed page: {:?}", pages.0[i]);
            totfixsum += pages.0[i][pages.0[i].len() / 2];
        }
    }
    println!("correct pages: {}", sum);
    println!("sum: {}", sum2);
    println!("pages to fix: {}", fixsum);
    println!("fixed pagenumbers: {:?}", pages.1);
    println!("totsum: {}", totfixsum - sum2);
}

fn check_pages(
    rules: (Vec<Vec<i32>>, Vec<Vec<bool>>),
    mut pages: (Vec<Vec<i32>>, Vec<i32>),
) -> Vec<i32> {
    for i in 0..pages.0.len() {
        pages.1[i] = check_page(rules.clone(), (pages.0[i].clone(), pages.1[i]));
    }
    pages.1
}

fn check_page(mut rules: (Vec<Vec<i32>>, Vec<Vec<bool>>), mut page: (Vec<i32>, i32)) -> i32 {
    page.1 = 0;
    for k in 0..rules.1.len() {
        rules.1[k][0] = false;
        rules.1[k][1] = false;
    }
    for j in 0..page.0.len() {
        for k in 0..rules.0.len() {
            if page.0[j] == rules.0[k][0] {
                if rules.1[k][1] {
                    page.1 = k as i32;
                    // break;
                } else {
                    rules.1[k][0] = true;
                }
            } else if page.0[j] == rules.0[k][1] {
                rules.1[k][1] = true;
                if rules.1[k][0] && page.1 == 0 {
                    page.1 = -1;
                }
            }
        }
        // println!("rules: {:?}", rules.1.clone());
    }
    // println!("{:?}", page.1);
    page.1
}

fn fix_page(rules: (Vec<Vec<i32>>, Vec<Vec<bool>>), mut page: (Vec<i32>, i32)) -> Vec<i32> {
    let mut places: (usize, usize) = (0, 0);
    for j in 0..page.0.len() {
        if rules.0[page.1 as usize][1] == page.0[j] {
            places.0 = j;
        } else if rules.0[page.1 as usize][0] == page.0[j] {
            places.1 = j;
        }
    }
    page.0.swap(places.0, places.1);
    page.0
}
