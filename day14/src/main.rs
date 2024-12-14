use std::fs;
fn main() {
    let file_path = "input/test.txt";
    let file_path = "input/day14.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{}", contents);

    let mut robots: Vec<Robot> = Vec::new();
    // let (width, height, moves) = (11, 7, 100);
    let (width, height, moves) = (101, 103, 100300000);

    contents.lines().for_each(|l| {
        // println!("{}", l);
        let mut robot = Robot {
            pos: (0, 0),
            vel: (0, 0),
        };
        l.split_whitespace().for_each(|w| {
            let (c, z) = w.split_once('=').unwrap();
            let (x, y) = z.split_once(',').unwrap();
            if c == "p" {
                robot.pos.0 = x.parse().unwrap();
                robot.pos.1 = y.parse().unwrap();
            } else {
                robot.vel.0 = x.parse().unwrap();
                robot.vel.1 = y.parse().unwrap();
            }
        });
        robots.push(robot);
    });

    // println!("{:?}", robots);

    // for _ in 0..moves {
    for i in 0..moves {
        robots.iter_mut().for_each(|r| {
            r.pos = get_next_pos(r.pos, r.vel, width, height);
        });
        print_map(robots.clone(), width, height, i);
    }
    let (mut ul, mut ur, mut dl, mut dr) = (0, 0, 0, 0);
    robots.iter().for_each(|r| {
        // println!("{:?}", r);
        if r.pos.0 < width / 2 && r.pos.1 < height / 2 {
            ul += 1;
        } else if r.pos.0 > width / 2 && r.pos.1 < height / 2 {
            ur += 1;
        } else if r.pos.0 < width / 2 && r.pos.1 > height / 2 {
            dl += 1;
        } else if r.pos.0 > width / 2 && r.pos.1 > height / 2 {
            dr += 1;
        }
    });
    println!("{} {} {} {}", ul, ur, dl, dr);
    println!("result: {}", ul * ur * dl * dr);
}

#[derive(Debug, Clone)]
struct Robot {
    pos: (i64, i64),
    vel: (i64, i64),
}

fn get_next_pos(pos: (i64, i64), vel: (i64, i64), width: i64, height: i64) -> (i64, i64) {
    let (mut x, mut y) = (pos.0 + vel.0, pos.1 + vel.1);
    if x < 0 {
        x = width + x;
    }
    if x >= width {
        x = x - width;
    }
    if y < 0 {
        y = height + y;
    }
    if y >= height {
        y = y - height;
    }
    (x, y)
}

// assume xmas tree has top middle and bottom middle
fn print_map(robots: Vec<Robot>, width: i64, height: i64, count: usize) {
    let (mut top, mut bottom, mut bleft, mut bright) = (false, false, false, false);
    let (mut left, mut right) = (0, 0);
    if count % 100000 == 0 {
        println!("count: {}", count);
    }
    // robots.iter().for_each(|r| {
    //     // if r.pos.1 == 0 && r.pos.0 == width / 2 {
    //     //     top = true;
    //     // }
    //     // if r.pos.1 == height - 1 && r.pos.0 == width / 2 {
    //     //     bottom = true;
    //     // }
    //     // if r.pos.1 == height - 1 && r.pos.0 == width / 2 - 1 {
    //     //     bleft = true;
    //     // }
    //     // if r.pos.1 == height - 1 && r.pos.0 == width / 2 + 1 {
    //     //     bright = true;
    //     // }
    //     if r.pos.0 < width / 2 {
    //         left += 1;
    //     }
    //     if r.pos.0 > width / 2 {
    //         right += 1;
    //     }
    // });
    // let adjacent_robots = robots
    //     .iter()
    //     .filter(|r| r.pos.1 == height - 1 && (r.pos.0 == width / 2 - 1 || r.pos.0 == width / 2 + 1))
    //     .count();
    let mut robstring = String::new();
    for y in 0..height {
        for x in 0..width {
            let mut c = '.';
            robots.iter().for_each(|r| {
                if r.pos.0 == x && r.pos.1 == y {
                    c = '#';
                }
            });
            robstring.push(c);
        }
        robstring = format!("{}\n", robstring);
        // println!("");
    }
    let print_out = robstring.contains("##########");
    // if left > robots.len() - 100 {
    //     bleft = true;
    // }
    // if right > robots.len() - 100 {
    //     bright = true;
    // }
    // if top && bottom && bleft && bright {
    // if bleft || bright {
    // if adjacent_robots > robots.len() / 2 {
    if print_out {
        println!("{}", robstring);
        // for y in 0..height {
        //     for x in 0..width {
        //         let mut c = '.';
        //         robots.iter().for_each(|r| {
        //             if r.pos.0 == x && r.pos.1 == y {
        //                 c = '#';
        //             }
        //         });
        //         print!("{}", c);
        //     }
        //     println!("");
        // }
        println!("count: {}", count + 1);
        std::io::stdin().read_line(&mut String::new());
    }
}
