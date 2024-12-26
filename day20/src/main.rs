use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::time::Instant;

fn bfs(
    map: &mut [bool],
    walls: &[(i32, i32)],
    start: (usize, usize),
    end: (usize, usize),
    size: (usize, usize),
    maxcheat: u32,
) -> usize {
    let mut dists = HashMap::new();
    map.fill(false);
    walls.iter().for_each(|w| {
        map[w.1 as usize * (size.0 + 1) + w.0 as usize] = true;
    });
    // for i in 0..drops {
    //     map[walls[i].1 as usize * (size.0 + 1) + walls[i].0 as usize] = true;
    // }
    let pos = (start.0 as i32, start.1 as i32, 0);
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    map[0] = true;

    while let Some((x, y, steps)) = queue.pop_front() {
        if dists.contains_key(&(x, y)) {
            continue;
        }
        dists.insert((x, y), steps as u32);
        if x == end.0 as i32 && y == end.1 as i32 {
            continue;
            // return Some(steps);
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            let si = ny as usize * (size.0 + 1) + nx as usize;
            if nx >= 0
                && ny >= 0
                && nx < (size.0 as i32 + 1)
                && ny < (size.1 as i32 + 1)
                && !map[si]
            {
                map[si] = true;
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }
    let mut score = 0;

    dists
        .iter()
        .tuple_combinations()
        .for_each(|(((x1, y1), s1), ((x2, y2), s2))| {
            let diff = x1.abs_diff(*x2) + y1.abs_diff(*y2);
            if diff <= maxcheat && s2.abs_diff(*s1) >= diff + 100 {
                score += 1;
            }
        });
    score
}

fn main() {
    let file_path = "input/test.txt";
    let now = Instant::now();
    let file_path = "input/day20.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut pathlen = 0;
    let mut walls: Vec<(i32, i32)> = Vec::new();
    let (mut start, mut end) = ((0, 0), (0, 0));
    contents.lines().enumerate().for_each(|(h, l)| {
        l.chars().enumerate().for_each(|(w, c)| {
            match c {
                '#' => {
                    walls.push((w as i32, h as i32));
                }
                'S' => {
                    start = (w, h);
                }
                'E' => {
                    end = (w, h);
                    pathlen += 1;
                }
                '.' => pathlen += 1,
                _ => (),
            };
        })
    });
    // let mut res: Vec<usize> = Vec::new();

    let width = walls.iter().map(|x| x.0).max().unwrap();
    let height = walls.iter().map(|x| x.1).max().unwrap();
    let mut map = vec![false; (width as usize + 1) * (height as usize + 1)];

    // for x in 1..width {
    //     for y in 1..height {
    //         if walls.contains(&(x, y)) {
    //             let mut cheatwalls = walls.clone();
    //             cheatwalls.swap_remove(walls.iter().position(|z| z == &(x, y)).unwrap());
    //
    //             res.push(
    //                 bfs(
    //                     &mut map,
    //                     &cheatwalls,
    //                     start,
    //                     end,
    //                     (width as usize, height as usize),
    //                 )
    //                 .unwrap(),
    //             );
    //
    //             // println!("x:{},y:{}", x, y);
    //         }
    //     }
    // }

    println!(
        "original time: {}, width: {}, height: {}",
        pathlen, width, height
    );

    let score = bfs(
        &mut map,
        &walls,
        start,
        end,
        (width as usize, height as usize),
        2,
    );
    println!("part 1: {}", score);
    let score = bfs(
        &mut map,
        &walls,
        start,
        end,
        (width as usize, height as usize),
        20,
    );
    println!("part 2: {}", score);
    // let mut goodcheats = 0;
    // res.iter().for_each(|x| {
    //     if *x < pathlen {
    //         // println!("{}", pathlen - x);
    //         if pathlen - x >= 100 {
    //             goodcheats += 1;
    //         }
    //     }
    // });
    // println!("res: {}", goodcheats);

    println!("time: {} microseconds", now.elapsed().as_micros());
}
