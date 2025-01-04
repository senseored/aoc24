use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Board {
    walls: Vec<(usize, usize)>,
    horse: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
    results: Vec<(i32, i32)>,
    prev_results: HashMap<(usize, usize), i32>,
    route: Vec<Vec<(usize, usize)>>,
    dirs: HashMap<(usize, usize), (bool, bool, bool, bool)>,
}
impl Board {
    fn new() -> Board {
        Board {
            walls: Vec::new(),
            horse: (0, 0),
            end: (0, 0),
            width: 0,
            height: 0,
            results: Vec::new(),
            prev_results: HashMap::new(),
            route: Vec::new(),
            dirs: HashMap::new(),
        }
    }
    // fn plug_dirs(&self, pos: (usize, usize)) -> bool {
    //     let mut x = 0;
    //     if !self.walls.contains(&(pos.0, pos.1 - 1)) {
    //         x += 1;
    //     }
    //     if !self.walls.contains(&(pos.0, pos.1 + 1)) {
    //         x += 1;
    //     }
    //     if !self.walls.contains(&(pos.0 - 1, pos.1)) {
    //         x += 1;
    //     }
    //     if !self.walls.contains(&(pos.1 + 1, pos.1)) {
    //         x += 1;
    //     }
    //     x == 1
    // }
    // fn plug_dead_ends(&mut self) {
    //     // let mut continue_loop = true;
    //     // let mut continue_loop2 = false;
    //     // while continue_loop {
    //     //     continue_loop2 = false;
    //     for x in 1..self.width - 1 {
    //         for y in 1..self.height - 1 {
    //             if !self.walls.contains(&(x, y)) {
    //                 if self.plug_dirs((x, y)) {
    //                     if self.end != (x, y) && self.horse != (x, y) {
    //                         print!("({}, {})", x, y);
    //                         self.walls.push((x, y));
    //                         // continue_loop2 = true;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     println!();
    //     // }
    //     // continue_loop = continue_loop2;
    // }
    fn draw_board(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.walls.contains(&(x, y)) {
                    print!("#");
                } else if (x, y) == self.horse {
                    print!("S");
                } else if (x, y) == self.end {
                    print!("E");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    fn get_dirs(&mut self, pos: (usize, usize)) -> (bool, bool, bool, bool) {
        let (l, r, u, d) = if self.dirs.contains_key(&pos) {
            self.dirs.get(&pos).unwrap()
        } else {
            let (ll, rr, uu, dd) = (
                !self.walls.contains(&(pos.0, pos.1 - 1)),
                !self.walls.contains(&(pos.0, pos.1 + 1)),
                !self.walls.contains(&(pos.0 - 1, pos.1)),
                !self.walls.contains(&(pos.0 + 1, pos.1)),
            );
            self.dirs.insert(pos, (ll, rr, uu, dd));
            &(ll, rr, uu, dd)
        };

        if !*l && !*r && !*u && *d
            || !*l && !*r && *u && !*d
            || !*l && *r && !*u && !*d
            || *l && !*r && !*u && !*d
        {
            if self.end != pos && self.horse != pos {
                self.walls.push(pos);
                if *l {
                    self.walls.push((pos.0, pos.1 - 1));
                }
                if *r {
                    self.walls.push((pos.0, pos.1 + 1));
                }
                if *u {
                    self.walls.push((pos.0 - 1, pos.1));
                }
                if *d {
                    self.walls.push((pos.0 + 1, pos.1));
                }
            }
        }
        (*l, *r, *u, *d)
    }
    fn return_dirs(&self, c: char, (l, r, u, d): (bool, bool, bool, bool)) -> (bool, bool, bool) {
        match c {
            '<' => (l, u, d),
            '>' => (r, u, d),
            '^' => (l, r, u),
            'v' => (l, r, d),
            _ => (true, true, true),
        }
    }
    fn move_horse(
        &mut self,
        c: char,
        pos: (usize, usize),
        results: (i32, i32),
        mut history: Vec<(usize, usize)>,
    ) -> bool {
        if self.end == pos {
            history.push(pos);
            self.results.push(results);
            self.route.push(history);
            if self.results.len() % 1000 == 0 {
                println!("results: {}", self.results.len());
            }
            return true;
        } else if history.contains(&pos) {
            // println!("visited");
            return false;
        } else if self.walls.contains(&pos) {
            // println!("wall");
            self.dirs.remove(&pos);
            self.dirs.remove(&(pos.0, pos.1 - 1));
            self.dirs.remove(&(pos.0, pos.1 + 1));
            self.dirs.remove(&(pos.0 - 1, pos.1));
            self.dirs.remove(&(pos.0 + 1, pos.1));
            return false;
        }
        if self.prev_results.contains_key(&pos) {
            if self.prev_results.get(&pos).unwrap() + 1000 < self.return_result(results) {
                // println!("visited w/better results");
                return false;
            } else {
                if self.prev_results.get(&pos).unwrap() > &self.return_result(results) {
                    self.prev_results.remove_entry(&pos);
                    self.prev_results.insert(pos, self.return_result(results));
                }
            }
        } else {
            self.prev_results
                .insert(pos, self.return_result(results) + 1001);
        }
        // println!("{} pos: {:?}", c, pos);
        history.push(pos);
        let (mut pos1, mut pos2, mut pos3) = (pos, pos, pos);
        let (mut results1, mut results2, mut results3) = (results, results, results);
        let (mut c1, mut c2, mut c3) = (c, c, c);
        match c {
            '<' => {
                (c1, c2, c3) = ('<', '^', 'v');
                (results1, results2, results3) = (
                    (results1.0 + 1, results1.1),
                    (results2.0 + 1, results2.1 + 1),
                    (results3.0 + 1, results3.1 + 1),
                );
                (pos1, pos2, pos3) = ((pos.0, pos.1 - 1), (pos.0 - 1, pos.1), (pos.0 + 1, pos.1));
            }
            '>' => {
                (c1, c2, c3) = ('>', '^', 'v');
                (results1, results2, results3) = (
                    (results1.0 + 1, results1.1),
                    (results2.0 + 1, results2.1 + 1),
                    (results3.0 + 1, results3.1 + 1),
                );
                (pos1, pos2, pos3) = ((pos.0, pos.1 + 1), (pos.0 - 1, pos.1), (pos.0 + 1, pos.1));
            }
            '^' => {
                (c1, c2, c3) = ('<', '>', '^');
                (results1, results2, results3) = (
                    (results1.0 + 1, results1.1 + 1),
                    (results2.0 + 1, results2.1 + 1),
                    (results3.0 + 1, results3.1),
                );
                (pos1, pos2, pos3) = ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1));
            }
            'v' => {
                (c1, c2, c3) = ('<', '>', 'v');
                (results1, results2, results3) = (
                    (results1.0 + 1, results1.1 + 1),
                    (results2.0 + 1, results2.1 + 1),
                    (results3.0 + 1, results3.1),
                );
                (pos1, pos2, pos3) = ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1), (pos.0 + 1, pos.1));
            }
            _ => (),
        };
        let (l, r, u, d) = self.get_dirs(pos);
        let (a, b, c) = self.return_dirs(c, (l, r, u, d));
        if a {
            self.move_horse(c1, pos1, results1, history.clone());
        }
        if b {
            self.move_horse(c2, pos2, results2, history.clone());
        }
        if c {
            self.move_horse(c3, pos3, results3, history.clone());
        }
        false
    }
    fn print_results(&self) {
        let mut result: Vec<i32> = Vec::new();
        let mut tiles: Vec<(usize, usize)> = Vec::new();
        self.results
            .iter()
            .for_each(|x| result.push(self.return_result(*x)));
        let lowest = result.iter().min().unwrap();
        let mut x: Vec<usize> = Vec::new();
        result.iter().enumerate().for_each(|(i, r)| {
            if r == lowest {
                x.push(i);
            }
        });
        // self.print_route(self.route[x[0]].clone());
        for i in 0..x.len() {
            self.route[x[i]].iter().for_each(|x| {
                if !tiles.contains(x) {
                    tiles.push(*x);
                }
            })
        }
        // println!("{:?}", self.route[x[x.len() - 1]]);
        // println!("results: {:?}", result);
        // println!("diff routes: {}", x.len());
        println!("results: {:?}, tiles: {}", lowest, tiles.len());
        // println!("results: {}", result.iter().min().unwrap());
    }
    // fn return_result_tiles(&self, result: (i32, i32)) -> (i32, i32, i32) {
    //     (result.0 + (result.1 * 1000), result.0, result.1)
    // }
    fn return_result(&self, result: (i32, i32)) -> i32 {
        result.0 + (result.1 * 1000)
    }
    fn print_route(&self, route: Vec<(usize, usize)>) {
        route.iter().for_each(|x| println!("{:?}", x));
    }
}
fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    let file_path = "input/day16.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{}", contents);
    // let lines: Vec<&str> = contents.split_whitespace().collect();
    // contents.split_whitespace().for_each(|x| println!("{}", x));

    let mut board = Board::new();

    contents.lines().enumerate().for_each(|(x, line)| {
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    board.walls.push((x as usize, y as usize));
                }
                'S' => {
                    board.horse = (x as usize, y as usize);
                }
                'E' => {
                    board.end = (x as usize, y as usize);
                }
                _ => (),
            }
        }
    });

    board.width = board.walls.iter().map(|x| x.0).max().unwrap() + 1;
    board.height = board.walls.iter().map(|x| x.1).max().unwrap() + 1;
    // board.draw_board();
    // board.plug_dead_ends();
    // board.draw_board();
    println!("horse: {:?}, end: {:?}", board.horse, board.end);
    board.move_horse('>', board.horse, (0, 0), Vec::new());
    board.draw_board();
    board.print_results();
}
