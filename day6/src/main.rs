use std::fs;

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/day6.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

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
    check_position(area.clone());
}

fn check_position(mut area: Vec<Vec<char>>) {
    let mut result = '.';
    for x in 0..area.len() {
        for y in 0..area[x].len() {
            match area[x][y] {
                '<' => result = area[x][y],
                '>' => result = area[x][y],
                'v' => result = area[x][y],
                '^' => {
                    result = area[x][y];
                    area[x][y] = 'X';
                    next_position(area.clone(), x, y, result);
                }
                _ => {}
            }
        }
    }
}

fn next_position(mut area: Vec<Vec<char>>, x: usize, y: usize, char: char) {
    match char {
        '<' => {
            // println!("go right: {} {}", x, y);
            for i in (0..y).rev() {
                // println!("{}", area[x][i]);
                match area[x][i] {
                    '#' => {
                        // println!("hit box");
                        area[x][i + 1] = 'X';
                        next_position(area.clone(), x, i + 1, '^');
                        break;
                    }
                    '.' | 'X' | '<' => {
                        area[x][i] = 'X';
                        if i == 0 {
                            get_result(area.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        '>' => {
            // println!("go left: {} {}", x, y);
            for i in y..area[x].len() {
                // println!("{}", area[x][i]);
                match area[x][i] {
                    '#' => {
                        // println!("hit box");
                        area[x][i - 1] = 'X';
                        next_position(area.clone(), x, i - 1, 'v');
                        break;
                    }
                    '.' | 'X' | '>' => {
                        area[x][i] = 'X';
                        if i == area[x].len() - 1 {
                            get_result(area.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        'v' => {
            // println!("go down: {} {}", x, y);
            for i in x..area.len() {
                // println!("{} {}", i, y);
                match area[i][y] {
                    '#' => {
                        // println!("hit box");
                        area[i - 1][y] = 'X';
                        next_position(area.clone(), i - 1, y, '<');
                        break;
                    }
                    '.' | 'X' | 'v' => {
                        area[i][y] = 'X';
                        if i == area.len() - 1 {
                            get_result(area.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
        '^' => {
            // println!("go up: {} {}", x, y);
            for i in (0..x).rev() {
                // println!("{} {}", i, y);
                match area[i][y] {
                    '#' => {
                        // println!("hit box");
                        area[i + 1][y] = 'X';
                        next_position(area.clone(), i + 1, y, '>');
                        break;
                    }
                    '.' | 'X' | '^' => {
                        area[i][y] = 'X';
                        if i == 0 {
                            get_result(area.clone());
                        }
                    }
                    _ => {}
                }
            }
        }
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
    for i in 0..area.len() {
        println!("{:?}", area[i]);
    }
    println!("result: {}", result);
}
