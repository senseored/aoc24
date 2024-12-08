use std::fs;

fn main() {
    // let file_path = "input/test.txt";
    let file_path = "input/day8.txt";
    // println!("In file {file_path}");

    let mut antennas: Vec<(char, Vec<Vec<i8>>)> = Vec::new();
    let mut antinodes: Vec<Vec<i8>> = Vec::new();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

    for (y, lines) in contents.lines().enumerate() {
        for (x, char) in lines.chars().enumerate() {
            if char != '.' {
                for i in 0..antennas.len() {
                    if antennas[i].0 == char {
                        antennas[i].1.push(vec![x as i8, y as i8]);
                    } else if i == antennas.len() - 1 {
                        antennas.push((char, vec![vec![x as i8, y as i8]]));
                    }
                }
                if antennas.is_empty() {
                    antennas.push((char, vec![vec![x as i8, y as i8]]));
                }
            }
        }
    }
    let height = contents.lines().count();
    let width = contents.lines().next().unwrap().len();

    for types in antennas {
        for (i, antenna) in types.1.iter().enumerate() {
            for j in 0..types.1.len() {
                if j != i {
                    let x: i8;
                    let y: i8;
                    x = antenna[0] + (antenna[0] - types.1[j][0]);
                    y = antenna[1] + (antenna[1] - types.1[j][1]);
                    if x >= 0 && x < width as i8 && y >= 0 && y < height as i8 {
                        let mut found = false;
                        for antinode in &antinodes {
                            if antinode[0] == x && antinode[1] == y {
                                found = true;
                            }
                        }
                        for antenna in &types.1 {
                            if antenna[0] == x && antenna[1] == y {
                                found = true;
                            }
                        }
                        if !found {
                            antinodes.push(vec![x, y]);
                        }
                    }
                }
            }
        }
    }
    println!("number of antinodes in map: {:?}", antinodes.len());
    // for x in 0..width {
    //     for y in 0..height {
    //         let mut found = false;
    //         for antinode in &antinodes {
    //             if antinode[0] == x as i8 && antinode[1] == y as i8 {
    //                 print!("#");
    //                 found = true;
    //             }
    //         }
    //         if !found {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
}
