use std::{fs, usize};
fn main() {
    // let file_path = "input/test.txt";
    let file_path = "input/day10.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let map = populate(&contents);

    search(map);
}
fn search(map: Vec<Vec<usize>>) {
    let trailheads = find_trailheads(map.clone());
    let mut sum = 0;
    let mut rating = 0;
    trailheads.iter().for_each(|start| {
        let results = check_route(*start, &map);
        rating += results.len();
        let score = calculate(results);
        sum += score;
    });
    println!("trailheads: {}", sum);
    println!("rating: {}", rating);
}
fn check_route(pos: (usize, usize), map: &Vec<Vec<usize>>) -> Vec<(bool, (usize, usize))> {
    let results: Vec<(bool, (usize, usize))> = Vec::new();
    fn recursive(
        mut results: Vec<(bool, (usize, usize))>,
        map: &Vec<Vec<usize>>,
        pos: (usize, usize),
        value: usize,
    ) -> Vec<(bool, (usize, usize))> {
        if value == 9 {
            // println!("found 9, pos: {:?} ", pos);
            results.push((true, pos));
        }
        // println!("attempt left");
        let pos2 = move_pos(pos, map[0].len() - 1, map.len() - 1, 'L');
        if map[pos2.0][pos2.1] == value + 1 {
            results = recursive(results.clone(), map, pos2, map[pos2.0][pos2.1]);
        }
        // println!("attempt right");
        let pos2 = move_pos(pos, map[0].len() - 1, map.len() - 1, 'R');
        if map[pos2.0][pos2.1] == value + 1 {
            results = recursive(results.clone(), map, pos2, map[pos2.0][pos2.1]);
        }
        // println!("attempt down");
        let pos2 = move_pos(pos, map[0].len() - 1, map.len() - 1, 'D');
        if map[pos2.0][pos2.1] == value + 1 {
            results = recursive(results.clone(), map, pos2, map[pos2.0][pos2.1]);
        }
        // println!("attempt up");
        let pos2 = move_pos(pos, map[0].len() - 1, map.len() - 1, 'U');
        if map[pos2.0][pos2.1] == value + 1 {
            results = recursive(results.clone(), map, pos2, map[pos2.0][pos2.1]);
        }
        results
    }
    recursive(results, &map, pos, map[pos.0][pos.1])
}
fn calculate(results: Vec<(bool, (usize, usize))>) -> usize {
    let mut unique_results: Vec<(bool, (usize, usize))> = Vec::new();
    for result in results.iter().enumerate() {
        if !unique_results.contains(result.1) {
            unique_results.push(result.1.clone());
        }
    }
    unique_results.len()
}

fn populate(contents: &str) -> Vec<Vec<usize>> {
    let mut map: Vec<Vec<usize>> = Vec::new();
    contents.lines().for_each(|line| {
        let mut mapline: Vec<usize> = Vec::new();
        line.chars().for_each(|char| {
            mapline.push(char.to_digit(10).unwrap() as usize);
        });
        map.push(mapline);
    });
    map
}

fn find_trailheads(map: Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut starts: Vec<(usize, usize)> = Vec::new();
    for (x, line) in map.iter().enumerate() {
        for (y, num) in line.iter().enumerate() {
            if *num == 0 {
                starts.push((x, y));
            }
        }
    }
    starts
}

fn move_pos(mut pos: (usize, usize), width: usize, height: usize, dir: char) -> (usize, usize) {
    match dir {
        'D' => {
            if pos.0 < height {
                pos.0 += 1
            }
        }
        'U' => {
            if pos.0 > 0 {
                pos.0 -= 1
            }
        }
        'R' => {
            if pos.1 < width {
                pos.1 += 1
            }
        }
        'L' => {
            if pos.1 > 0 {
                pos.1 -= 1
            }
        }
        _ => {}
    };
    pos
}
