use std::collections::HashMap;
use std::fs;

enum Direction {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
}

#[derive(Debug, Clone)]
struct Map {
    walls: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
    // results: Vec<(i32, i32)>,
    prev_results: HashMap<(usize, usize), usize>,
    route: Vec<Vec<(usize, usize)>>,
    shortest: usize,
    dirs: HashMap<(usize, usize), (bool, bool, bool, bool)>,
    // dirs: HashMap<(usize, usize), (usize, usize, usize, usize)>,
}
impl Map {
    fn new() -> Map {
        Map {
            walls: Vec::new(),
            start: (0, 0),
            end: (0, 0),
            width: 0,
            height: 0,
            // results: Vec::new(),
            prev_results: HashMap::new(),
            route: Vec::new(),
            shortest: 0,
            dirs: HashMap::new(),
        }
    }
    fn draw_best_route(&self, walls: usize) {
        let mut z = 0;
        self.route.iter().enumerate().for_each(|(i, r)| {
            if r.len() == self.shortest {
                z = i;
            }
        });
        for x in 0..self.width {
            for y in 0..self.height {
                if self.walls[0..walls].contains(&(x, y)) {
                    print!("#");
                } else if (x, y) == self.start {
                    print!("S");
                } else if (x, y) == self.end {
                    print!("E");
                } else if self.route[z].contains(&(x, y)) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    fn draw_map(&self, walls: usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.walls[0..walls].contains(&(x, y)) {
                    print!("#");
                } else if (x, y) == self.start {
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
    fn get_dirs(&mut self, pos: (usize, usize), walls: usize) -> (bool, bool, bool, bool) {
        let (l, r, u, d) = if self.dirs.contains_key(&pos) {
            self.dirs.get(&pos).unwrap()
        } else {
            let lpos = if pos.1 > 0 { pos.1 - 1 } else { pos.1 };
            let rpos = if pos.1 < self.width - 1 {
                pos.1 + 1
            } else {
                pos.1
            };
            let upos = if pos.0 > 0 { pos.0 - 1 } else { pos.0 };
            let dpos = if pos.0 < self.height - 1 {
                pos.0 + 1
            } else {
                pos.0
            };
            let (ll, rr, uu, dd) = (
                !self.walls[0..walls].contains(&(pos.0, lpos)),
                !self.walls[0..walls].contains(&(pos.0, rpos)),
                !self.walls[0..walls].contains(&(upos, pos.1)),
                !self.walls[0..walls].contains(&(dpos + 1, pos.1)),
            );
            self.dirs.insert(pos, (ll, rr, uu, dd));
            &(ll, rr, uu, dd)
        };

        println!("l: {} - r: {} - u: {} - d: {}", l, r, u, d);
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
    fn do_move(
        &mut self,
        c: char,
        pos: (usize, usize),
        walls: usize,
        mut history: Vec<(usize, usize)>,
    ) -> bool {
        if self.end == pos {
            self.route.push(history.clone());
            if history.len() < self.shortest || self.shortest == 0 {
                self.shortest = history.len();
                println!("shortest: {}", self.shortest);
            }
            return true;
            // history.push(pos);
            // self.results.push(results);
            // self.route.push(history);
            // if self.results.len() % 1000 == 0 {
            //     println!("results: {}", self.results.len());
            // }
            // return true;
        } else if history.contains(&pos) {
            // println!("visited");
            return false;
        } else if self.walls[0..walls].contains(&pos) {
            // println!("wall");
            self.dirs.remove(&pos);
            return false;
        }
        if self.prev_results.contains_key(&pos) {
            // println!("new position: {:?}", pos);
            if self.prev_results.get(&pos).unwrap() <= &(history.len()) {
                // println!("better results before");
                return false;
            } else {
                if self.prev_results.get(&pos).unwrap() > &(history.len()) {
                    self.prev_results.remove_entry(&pos);
                    self.prev_results.insert(pos, history.len());
                }
            }
        } else {
            self.prev_results.insert(pos, history.len());
        }
        // if self.prev_results.contains_key(&pos) {
        //     if self.prev_results.get(&pos).unwrap() + 1000 < self.return_result(results) {
        //         // println!("visited w/better results");
        //         return false;
        //     } else {
        //         if self.prev_results.get(&pos).unwrap() > &self.return_result(results) {
        //             self.prev_results.remove_entry(&pos);
        //             self.prev_results.insert(pos, self.return_result(results));
        //         }
        //     }
        // } else {
        //     self.prev_results
        //         .insert(pos, self.return_result(results) + 1001);
        // }
        println!("{} pos: {:?}", c, pos);
        history.push(pos);
        let (mut pos1, mut pos2, mut pos3) = (pos, pos, pos);
        let (mut c1, mut c2, mut c3) = (c, c, c);
        match c {
            '<' => {
                (c1, c2, c3) = ('<', '^', 'v');
                (pos1, pos2, pos3) = ((pos.0, pos.1 - 1), (pos.0 - 1, pos.1), (pos.0 + 1, pos.1));
            }
            '>' => {
                (c1, c2, c3) = ('>', '^', 'v');
                (pos1, pos2, pos3) = ((pos.0, pos.1 + 1), (pos.0 - 1, pos.1), (pos.0 + 1, pos.1));
            }
            '^' => {
                (c1, c2, c3) = ('<', '>', '^');
                (pos1, pos2, pos3) = ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1));
            }
            'v' => {
                (c1, c2, c3) = ('<', '>', 'v');
                (pos1, pos2, pos3) = ((pos.0, pos.1 - 1), (pos.0, pos.1 + 1), (pos.0 + 1, pos.1));
            }
            _ => (),
        };
        let (l, r, u, d) = self.get_dirs(pos, walls);
        let (a, b, c) = self.return_dirs(c, (l, r, u, d));
        if a {
            self.do_move(c1, pos1, walls, history.clone());
        }
        if b {
            self.do_move(c2, pos2, walls, history.clone());
        }
        if c {
            self.do_move(c3, pos3, walls, history.clone());
        }
        false
    }
    // fn get_dirs(
    //     &mut self,
    //     pos: (usize, usize),
    //     walls: usize,
    //     history: Vec<(usize, usize)>,
    // ) -> (usize, usize, usize, usize) {
    //     let (l, r, u, d) = if self.dirs.contains_key(&pos) {
    //         self.dirs.get(&pos).unwrap()
    //     } else {
    //         let lpos = if pos.1 > 0 { pos.1 - 1 } else { pos.1 };
    //         let rpos = if pos.1 < self.width - 1 {
    //             pos.1 + 1
    //         } else {
    //             pos.1
    //         };
    //         let upos = if pos.0 > 0 { pos.0 - 1 } else { pos.0 };
    //         let dpos = if pos.0 < self.height - 1 {
    //             pos.0 + 1
    //         } else {
    //             pos.0
    //         };
    //
    //         let (ll, rr, uu, dd) = (
    //             if !self.walls[0..walls].contains(&(pos.0, lpos))
    //                 && !history.contains(&(pos.0, lpos))
    //             {
    //                 lpos
    //             } else {
    //                 pos.1
    //             },
    //             if !self.walls[0..walls].contains(&(pos.0, rpos))
    //                 && !history.contains(&(pos.0, rpos))
    //             {
    //                 rpos
    //             } else {
    //                 pos.1
    //             },
    //             if !self.walls[0..walls].contains(&(upos, pos.1))
    //                 && !history.contains(&(upos, pos.1))
    //             {
    //                 upos
    //             } else {
    //                 pos.0
    //             },
    //             if !self.walls[0..walls].contains(&(dpos, pos.1))
    //                 && !history.contains(&(dpos, pos.1))
    //             {
    //                 dpos
    //             } else {
    //                 pos.0
    //             },
    //         );
    //         self.dirs.insert(pos, (ll, rr, uu, dd));
    //         &(ll, rr, uu, dd)
    //     };
    //
    //     (*l, *r, *u, *d)
    // }
    // // fn return_dirs(
    // //     &self,
    // //     c: char,
    // //     (l, r, u, d): (usize, usize, usize, usize),
    // // ) -> (usize, usize, usize) {
    // //     match c {
    // //         '<' => (l, u, d),
    // //         '>' => (r, u, d),
    // //         '^' => (l, r, u),
    // //         'v' => (l, r, d),
    // //         _ => (0, 0, 0),
    // //     }
    // // }
    //
    // fn do_move(
    //     &mut self,
    //     c: char,
    //     pos: (usize, usize),
    //     walls: usize,
    //     mut history: Vec<(usize, usize)>,
    // ) -> bool {
    //     println!("pos: {:?} - history: {:?} - c: {}", pos, history, c);
    //     if self.end == pos {
    //         // history.push(pos);
    //         self.route.push(history.clone());
    //         if history.len() < self.shortest || self.shortest == 0 {
    //             self.shortest = history.len();
    //             println!("shortest: {}", self.shortest);
    //         }
    //         return true;
    //     } else if history.contains(&pos) {
    //         return false;
    //     }
    //     history.push(pos);
    //     println!("after hispuch");
    //     if self.prev_results.contains_key(&pos) {
    //         // println!("new position: {:?}", pos);
    //         if self.prev_results.get(&pos).unwrap() <= &(history.len()) {
    //             // println!("better results before");
    //             return false;
    //         } else {
    //             if self.prev_results.get(&pos).unwrap() > &(history.len()) {
    //                 self.prev_results.remove_entry(&pos);
    //                 self.prev_results.insert(pos, history.len());
    //             }
    //         }
    //     } else {
    //         self.prev_results.insert(pos, history.len());
    //     }
    //     // println!("history_len: {} - pos: {:?}", history.len(), pos);
    //     // println!("new position: {:?}", pos);
    //     let (l, r, u, d) = self.get_dirs(pos, walls, history.clone());
    //     let (mut pos1, mut pos2, mut pos3) = (pos, pos, pos);
    //     let (mut c1, mut c2, mut c3) = (c, c, c);
    //     let (mut x, mut y, mut z) = (false, false, false);
    //     match c {
    //         '<' => {
    //             (c1, c2, c3) = ('<', '^', 'v');
    //             pos1 = (pos.0, l);
    //             pos2 = (u, pos.1);
    //             pos3 = (d, pos.1);
    //             x = l != pos.1;
    //             y = u != pos.0;
    //             z = d != pos.0;
    //         }
    //         '>' => {
    //             (c1, c2, c3) = ('>', '^', 'v');
    //             pos1 = (pos.0, r);
    //             pos2 = (u, pos.1);
    //             pos3 = (d, pos.1);
    //             x = r != pos.1;
    //             y = u != pos.0;
    //             z = d != pos.0;
    //         }
    //         '^' => {
    //             (c1, c2, c3) = ('<', '>', '^');
    //             pos1 = (pos.0, l);
    //             pos2 = (pos.0, r);
    //             pos3 = (u, pos.1);
    //             x = l != pos.1;
    //             y = r != pos.1;
    //             z = u != pos.0;
    //         }
    //         'v' => {
    //             (c1, c2, c3) = ('<', '>', 'v');
    //             pos1 = (pos.0, l);
    //             pos2 = (pos.0, r);
    //             pos3 = (d, pos.1);
    //             x = l != pos.1;
    //             y = r != pos.1;
    //             z = d != pos.0;
    //         }
    //         _ => (),
    //     };
    //     println!("x: {} - y: {} - z: {}", x, y, z);
    //     if z {
    //         return self.do_move(c3, pos3, walls, history.clone());
    //     }
    //     if x {
    //         return self.do_move(c1, pos1, walls, history.clone());
    //     }
    //     if y {
    //         return self.do_move(c2, pos2, walls, history.clone());
    //     }
    //     false
    //     // self.do_move('<', (pos.0, l), walls, history.clone());
    //     // self.do_move('>', (pos.0, r), walls, history.clone());
    //     // self.do_move('^', (u, pos.1), walls, history.clone());
    //     // self.do_move('v', (d, pos.1), walls, history.clone());
    //     // println!("l: {} - r: {} - u: {} - d: {}", l, r, u, d);
    //     // println!("pos: {:?} - history: {:?} - c: {}", pos, history, c);
    //     // self.do_move('<', (pos.0, l), walls, history.clone())
    //     //     && self.do_move('>', (pos.0, r), walls, history.clone())
    //     //     && self.do_move('^', (u, pos.1), walls, history.clone())
    //     //     && self.do_move('v', (d, pos.1), walls, history.clone())
    //     // false
    // }
    fn print_shortest_route(&self) {
        println!("{}", self.shortest);
        // self.route.iter().for_each(|x| {
        // if x.len() == self.shortest {
        //     println!("{:?}", x);
        // }
        // });
    }
}

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/test2.txt";
    // let file_path = "input/day18.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut map: Map = Map::new();

    map.width = 7;
    let walls = 12;
    // let walls = 1024;
    // map.width = 71;
    map.height = map.width;
    map.end = (map.width - 1, map.height - 1);

    contents.lines().for_each(|a| {
        let (y, x) = a.split_once(',').unwrap();
        map.walls
            .push((x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()));
    });
    // println!("{:?}", map.walls);

    // map.draw_map(12);
    // map.do_move('>', map.start, 12, Vec::new());
    map.draw_map(walls);
    map.do_move('>', map.start, walls, Vec::new());
    map.draw_best_route(walls);
    map.print_shortest_route();

    // map.walls = contents.lines().for_each(|x| x.split(',').map(|x| x.parse::usize().unwrap()).collect()).collect();

    // println!("{}", contents);
    // let lines: Vec<&str> = contents.split_whitespace().collect();
    // contents.split_whitespace().for_each(|x| println!("{}", x));
}
