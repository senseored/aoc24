// use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

fn main() {
    let file_path = "input/test.txt";
    let file_path = "input/day13.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");
    let machines = parse_input(contents);
    let count = 100;
    let mut results: Vec<(i64, i64)> = Vec::new();
    machines.iter().enumerate().for_each(|(i, m)| {
        // println!("id: {}", i);
        // let results = part1(&mut m.clone(), count, i);
        for x in 0..count {
            for y in 0..count {
                if ((x as i64 * m.a.0 + y as i64 * m.b.0) == m.prize.0)
                    && ((x as i64 * m.a.1 + y as i64 * m.b.1) == m.prize.1)
                {
                    results.push((i as i64, (x * 3 + y) as i64));
                }
            }
        }
        // results.iter().for_each(|r| println!("{:?}", r))
    });
    let mut tot = 0;
    results.iter().for_each(|r| tot += r.1);
    println!("part1: {}", tot);
    let mut results: Vec<(i64, i64)> = Vec::new();
    machines.iter().enumerate().for_each(|(i, m)| {
        let (ax, ay, bx, by, px, py) = (
            m.a.0,
            m.a.1,
            m.b.0,
            m.b.1,
            m.prize.0 + 10000000000000,
            m.prize.1 + 10000000000000,
        );

        let num_b = px * ay - py * ax;
        let den_b = bx * ay - by * ax;

        if den_b != 0 && num_b % den_b == 0 {
            let b = num_b / den_b;

            let num_a = px - b * bx;
            let den_a = ax;
            if den_a != 0 && num_a % den_a == 0 {
                let a = num_a / den_a;
                results.push((i as i64, a * 3 + b));
            }
        }
    });
    let mut tot = 0;
    results.iter().for_each(|r| tot += r.1);
    println!("part2: {}", tot);
}

fn parse_input(contents: String) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let mut m = Machine {
        a: (0, 0),
        b: (0, 0),
        prize: (0, 0),
    };
    contents.lines().for_each(|l| {
        if l.starts_with("Button A: X+") {
            let (x, y) = l
                .split_once("Button A: X+")
                .unwrap()
                .1
                .split_once(", Y+")
                .unwrap();
            m.a = (x.parse().unwrap(), y.parse().unwrap());
        } else if l.starts_with("Button B: X+") {
            let (x, y) = l
                .split_once("Button B: X+")
                .unwrap()
                .1
                .split_once(", Y+")
                .unwrap();
            m.b = (x.parse().unwrap(), y.parse().unwrap());
        } else if l.starts_with("Prize: X=") {
            let (x, y) = l
                .split_once("Prize: X=")
                .unwrap()
                .1
                .split_once(", Y=")
                .unwrap();
            m.prize = (x.parse().unwrap(), y.parse().unwrap());
            machines.push(m.clone());
        }
    });
    machines
}
