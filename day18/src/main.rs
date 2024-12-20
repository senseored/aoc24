use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

fn bfs(map: &mut [bool], drops: usize, walls: &[(i32, i32)], size: usize) -> Option<usize> {
    map.fill(false);
    for i in 0..drops {
        map[walls[i].1 as usize * (size + 1) + walls[i].0 as usize] = true;
    }
    let pos = (0, 0, 0);
    let mut queue = VecDeque::new();
    queue.push_back(pos);
    map[0] = true;

    while let Some((x, y, steps)) = queue.pop_front() {
        if x == size as i32 && y == size as i32 {
            return Some(steps);
        }
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let nx = x + dx;
            let ny = y + dy;
            let si = ny as usize * (size + 1) + nx as usize;
            if nx >= 0 && ny >= 0 && nx < (size as i32 + 1) && ny < (size as i32 + 1) && !map[si] {
                map[si] = true;
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }
    None
}

fn main() {
    // let file_path = "input/test.txt";
    // let drops = 12;
    // let size = 8;
    // let file_path = "input/test2.txt";
    use std::time::Instant;
    let now = Instant::now();
    let file_path = "input/day18.txt";
    let drops = 1024;
    let size = 70;

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut walls: Vec<(i32, i32)> = Vec::new();
    contents.lines().for_each(|a| {
        let (x, y) = a.split_once(',').unwrap();
        walls.push((x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()));
    });

    let mut map = vec![false; (size + 1) * (size + 1)];
    let res1 = bfs(&mut map, drops, &walls, size).unwrap();

    println!("part1: {}", res1);

    let mut min = drops + 1;
    let mut max = walls.len();

    while min != max {
        let mid = (min + max) / 2;
        if bfs(&mut map, mid, &walls, size).is_some() {
            min = mid + 1;
        } else {
            max = mid;
        }
    }

    println!("part2: {},{}", walls[max - 1].0, walls[max - 1].1);

    println!("time: {} microseconds", now.elapsed().as_micros());
}
