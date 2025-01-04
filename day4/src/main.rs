use std::fs;

fn main() {
    // --snip--
    // let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    let file_path = "input/day4.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut sum = 0;
    let mut seq: Vec<Vec<char>> = Vec::new();

    contents.lines().for_each(|line| {
        let mut l = Vec::new();
        for char in line.chars() {
            l.push(char);
        }
        seq.push(l);
    });
    let (width, height) = (seq.len(), seq[0].len());

    for y in 0..height {
        for x in 0..width {
            if seq[y][x] == 'X' {
                //horizontal ->
                if x < width - 3
                    && check_letter(seq.clone(), x + 1, y, 'M')
                    && check_letter(seq.clone(), x + 2, y, 'A')
                    && check_letter(seq.clone(), x + 3, y, 'S')
                {
                    sum += 1;
                }
                //horizontal <-
                if x > 2 {
                    if check_letter(seq.clone(), x - 1, y, 'M') {
                        if check_letter(seq.clone(), x - 2, y, 'A') {
                            if check_letter(seq.clone(), x - 3, y, 'S') {
                                sum += 1;
                            }
                        }
                    }
                }
                //vertical ^
                if y > 2 {
                    if check_letter(seq.clone(), x, y - 1, 'M') {
                        if check_letter(seq.clone(), x, y - 2, 'A') {
                            if check_letter(seq.clone(), x, y - 3, 'S') {
                                sum += 1;
                            }
                        }
                    }
                }
                //vertical v
                if y < height - 3 {
                    if check_letter(seq.clone(), x, y + 1, 'M') {
                        if check_letter(seq.clone(), x, y + 2, 'A') {
                            if check_letter(seq.clone(), x, y + 3, 'S') {
                                sum += 1;
                            }
                        }
                    }
                }
                //diagonal LD
                if y < height - 3 && x < width - 3 {
                    if check_letter(seq.clone(), x + 1, y + 1, 'M') {
                        if check_letter(seq.clone(), x + 2, y + 2, 'A') {
                            if check_letter(seq.clone(), x + 3, y + 3, 'S') {
                                sum += 1;
                            }
                        }
                    }
                }
                //diagonal LU
                if y < height - 3 && x > 2 {
                    if check_letter(seq.clone(), x - 1, y + 1, 'M') {
                        if check_letter(seq.clone(), x - 2, y + 2, 'A') {
                            if check_letter(seq.clone(), x - 3, y + 3, 'S') {
                                sum += 1;
                            }
                        }
                    }
                }
                //diagonal RD
                if y > 2 && x < width - 3 {
                    if check_letter(seq.clone(), x + 1, y - 1, 'M') {
                        if check_letter(seq.clone(), x + 2, y - 2, 'A') {
                            if check_letter(seq.clone(), x + 3, y - 3, 'S') {
                                sum += 1;
                            }
                        }
                    }
                }
                //diagonal RU
                if y > 2 && x > 2 {
                    if check_letter(seq.clone(), x - 1, y - 1, 'M') {
                        if check_letter(seq.clone(), x - 2, y - 2, 'A') {
                            if check_letter(seq.clone(), x - 3, y - 3, 'S') {
                                sum += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("test 1: {}", sum);

    sum = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if seq[y][x] == 'A' && mas_check(seq.clone(), x, y) {
                sum += 1;
            }
        }
    }
    println!("test 2: {}", sum);
}

fn mas_check(seq: Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if ((seq[y + 1][x + 1] == 'M' && seq[y - 1][x - 1] == 'S')
        || (seq[y + 1][x + 1] == 'S' && seq[y - 1][x - 1] == 'M'))
        && ((seq[y - 1][x + 1] == 'M' && seq[y + 1][x - 1] == 'S')
            || (seq[y - 1][x + 1] == 'S' && seq[y + 1][x - 1] == 'M'))
    {
        return true;
    }
    false
}
fn check_letter(seq: Vec<Vec<char>>, x: usize, y: usize, char: char) -> bool {
    if seq[y][x] == char {
        return true;
    }
    false
}
