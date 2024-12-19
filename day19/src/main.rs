use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    // let file_path = "input/test.txt";
    let file_path = "input/day19.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // println!("{}", contents);

    let (towels, patterns) = contents.split_once("\n\n").unwrap();
    let towels: Vec<String> = towels.split(", ").map(String::from).collect();
    let patterns: Vec<String> = patterns.lines().map(String::from).collect();
    // let mut possible = vec![false; patterns.len()];
    let mut hm: HashMap<String, usize> = HashMap::new();

    // println!("{:?}", patterns);
    //
    // println!("{:?}", towels);

    println!(
        "possible towels: {}",
        patterns
            .iter()
            .filter(|p| combinations(&mut hm, p, &towels) > 0)
            .count()
    );

    println!(
        "possible combinations: {}",
        patterns
            .iter()
            .map(|p| combinations(&mut hm, p, &towels))
            .sum::<usize>()
    );
    println!("time: {} microseconds", now.elapsed().as_micros());
}

fn combinations(hm: &mut HashMap<String, usize>, pattern: &str, towels: &Vec<String>) -> usize {
    if hm.contains_key(pattern) {
        return *hm.get(pattern).unwrap();
    }
    let result = towels
        .iter()
        .filter(|t| pattern.starts_with(*t))
        .map(|t| {
            let (_, p) = pattern.split_at(t.len());
            if p.len() == 0 {
                1
            } else {
                combinations(hm, p, towels)
            }
        })
        .sum();

    hm.insert(pattern.to_string(), result);

    result
}
