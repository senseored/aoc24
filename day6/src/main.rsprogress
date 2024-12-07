use std::fs;

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/day6.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut area: Vec<Vec<char>> = Vec::new();

    for lines in contents.lines() {
        let mut line = Vec::new();
        for char in lines.chars() {
            line.push(char);
        }
        area.push(line);
    }

    // println!("{:?}", area);
    // println!("{}", check_position(area.clone()));
    check_start_position(area.clone());
}

fn check_start_position(area: Vec<Vec<char>>) {
    let mut count = 0;
    for i in 0..area.len() {
        for j in 0..area[i].len() {
            match area[i][j] {
                '<' | '>' | '^' | 'v' => {
                    change_position(area.clone(), i, j, area[i][j], 0);
                }
                _ => {}
            }
        }
    }
}

fn change_position(mut area: Vec<Vec<char>>, i: usize, j: usize, char: char, mut loops: i32) {
    // for i in 0..area.len() {
    //     println!("{:?}", area[i]);
    // }
    println!("i: {}, j: {}, char: {}", i, j, char);
    match char {
        '<' => {
            if i == area.len() - 1 {
                get_result(area.clone());
            } else if area[i][j - 1] == '#' {
                area[i][j] = '^';
                change_position(area.clone(), i, j, '^', loops);
            } else {
                area[i][j] = 'X';
                area[i][j - 1] = '<';
                change_position(area.clone(), i, j - 1, char, loops);
            }
        }
        '>' => {
            if j == area.len() - 1 {
                get_result(area.clone());
            } else if area[i][j + 1] == '#' {
                area[i][j] = 'v';
                change_position(area.clone(), i, j, 'v', loops);
            } else {
                area[i][j] = 'X';
                area[i][j + 1] = '>';
                change_position(area.clone(), i, j + 1, char, loops);
            }
        }
        '^' => {
            if i == 0 {
                get_result(area.clone());
            } else if area[i - 1][j] == '#' {
                area[i][j] = '>';
                change_position(area.clone(), i, j, '>', loops);
            } else {
                area[i][j] = 'X';
                area[i - 1][j] = '^';
                change_position(area.clone(), i - 1, j, char, loops);
            }
        }
        'v' => {
            if i == area.len() - 1 {
                get_result(area.clone());
            } else if area[i + 1][j] == '#' {
                area[i][j] = '<';
                change_position(area.clone(), i, j, '<', loops);
            } else {
                area[i][j] = 'X';
                area[i + 1][j] = 'v';
                change_position(area.clone(), i + 1, j, char, loops);
            }
        }
        'X' => {}
        '#' => {}
        _ => {}
    }
}

fn get_result(area: Vec<Vec<char>>) {
    let mut result = 0;
    for i in 0..area.len() {
        for j in 0..area[i].len() {
            if area[i][j] == 'X' {
                result += 1;
            }
        }
    }
    // for i in 0..area.len() {
    //     println!("{:?}", area[i]);
    // }
    println!("result: {}", result + 1);
}
fn loop_check(
    mut area: Vec<Vec<char>>,
    i: usize,
    j: usize,
    char: char,
    start_i: usize,
    start_j: usize,
) {
    // for i in 0..area.len() {
    //     println!("{:?}", area[i]);
    // }
    println!("i: {}, j: {}, char: {}", i, j, char);
    match char {
        '<' => {
            if i == area.len() - 1 {
                get_result(area.clone());
            } else if area[i][j - 1] == '#' {
                area[i][j] = '^';
                loop_check(area.clone(), i, j, '^', start_i, start_j);
            } else {
                area[i][j] = 'X';
                area[i][j - 1] = '<';
                loop_check(area.clone(), i, j - 1, char, start_i, start_j);
            }
        }
        '>' => {
            if j == area.len() - 1 {
                get_result(area.clone());
            } else if area[i][j + 1] == '#' {
                area[i][j] = 'v';
                loop_check(area.clone(), i, j, 'v', start_i, start_j);
            } else {
                area[i][j] = 'X';
                area[i][j + 1] = '>';
                loop_check(area.clone(), i, j + 1, char, start_i, start_j);
            }
        }
        '^' => {
            if i == 0 {
                get_result(area.clone());
            } else if area[i - 1][j] == '#' {
                area[i][j] = '>';
                loop_check(area.clone(), i, j, '>', start_i, start_j);
            } else {
                area[i][j] = 'X';
                area[i - 1][j] = '^';
                loop_check(area.clone(), i - 1, j, char, start_i, start_j);
            }
        }
        'v' => {
            if i == area.len() - 1 {
                get_result(area.clone());
            } else if area[i + 1][j] == '#' {
                area[i][j] = '<';
                loop_check(area.clone(), i, j, '<', start_i, start_j);
            } else {
                area[i][j] = 'X';
                area[i + 1][j] = 'v';
                loop_check(area.clone(), i + 1, j, char, start_i, start_j);
            }
        }
        'X' => {}
        '#' => {}
        _ => {}
    }
}
