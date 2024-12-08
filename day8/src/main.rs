use std::fs;

fn main() {
    // let file_path = "input/test2.txt";
    // let file_path = "input/test.txt";
    let file_path = "input/day8.txt";
    // println!("In file {file_path}");

    let mut antennas: Vec<(char, Vec<Vec<i8>>)> = Vec::new();
    let mut antinodes: Vec<Vec<i8>> = Vec::new();
    let mut antinodes2: Vec<Vec<i8>> = Vec::new();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");

    for (x, lines) in contents.lines().enumerate() {
        for (y, char) in lines.chars().enumerate() {
            if char != '.' {
                println!("{} {} {}", char, x, y);
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

    for types in &antennas {
        for (i, antenna) in types.1.iter().enumerate() {
            if types.1.len() > 1 {
                let x = antenna[0];
                let y = antenna[1];
                let mut found3 = false;
                for antinode in &antinodes2 {
                    if antinode[0] == x && antinode[1] == y {
                        println!("found2 dupe, x: {}, y: {}", x, y);
                        found3 = true;
                    }
                }
                if !found3 {
                    antinodes2.push(vec![antenna[0], antenna[1]]);
                }
            }
            for j in 0..types.1.len() {
                if j != i {
                    let mut x: i8;
                    let mut y: i8;
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
                        while x >= 0 && x < width as i8 && y >= 0 && y < height as i8 {
                            let mut found2 = false;
                            println!(
                                "trying, x: {}, y: {}, antinode2.len: {}",
                                x,
                                y,
                                antinodes2.len()
                            );
                            for antinode in &antinodes2 {
                                if antinode[0] == x && antinode[1] == y {
                                    found2 = true;
                                    println!("found2 dupe, x: {}, y: {}", x, y);
                                }
                            }
                            // for antenna in &types.1 {
                            //     if antenna[0] == x && antenna[1] == y {
                            //         found2 = true;
                            //     }
                            // }
                            if !found2 {
                                antinodes2.push(vec![x, y]);
                            }
                            x = x + (antenna[0] - types.1[j][0]);
                            y = y + (antenna[1] - types.1[j][1]);
                        }
                    }
                }
            }
        }
    }
    println!("number of antinodes in map: {:?}", antinodes.len());
    println!("number of antinodes2 in map: {:?}", antinodes2.len());
    let mut count = 0;
    // for x in 0..width {
    //     for y in 0..height {
    //         let mut found = false;
    //         for antenna in &antennas[0].1.clone() {
    //             if antenna[0] == x as i8 && antenna[1] == y as i8 {
    //                 print!("{}", antennas[0].0);
    //                 // count += 1;
    //                 found = true;
    //             }
    //         }
    //         if !found {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    for x in 0..width {
        for y in 0..height {
            let mut found = false;
            for antinode2 in &antinodes2 {
                if antinode2[0] == x as i8 && antinode2[1] == y as i8 {
                    print!("#");
                    count += 1;
                    found = true;
                }
            }
            if !found {
                print!(".");
            }
        }
        println!();
    }
    println!("antinodes2: {:?}", antinodes2);
    println!("count: {}", count);

    for (i, antinode) in antinodes2.iter().enumerate() {
        for j in i + 1..antinodes2.len() {
            if antinode[0] == antinodes2[j][0] && antinode[1] == antinodes2[j][1] {
                count -= 1;
            }
        }
    }
    println!("count: {}", count);
}
