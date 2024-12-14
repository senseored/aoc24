use std::collections::HashMap;
use std::fs;

#[derive(Clone)]
struct State {
    map: Vec<Vec<char>>,
    plots: HashMap<i32, i32>,
    fence: HashMap<i32, i32>,
    sides: HashMap<i32, i32>,
    checked: Vec<Vec<bool>>,
}
impl State {
    fn new() -> State {
        State {
            map: Vec::new(),
            plots: HashMap::new(),
            fence: HashMap::new(),
            sides: HashMap::new(),
            checked: Vec::new(),
        }
    }
    fn add_map(&mut self, contents: String) {
        self.map = contents
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        self.checked = vec![vec![false; self.map[0].len()]; self.map.len()];
        // contents.lines().for_each(|line| {
        //     line.chars().for_each(|c| {
        //         self.plots.entry(c as i32).or_insert(0);
        //         self.fence.entry(c as i32).or_insert(0);
        //         self.sides.entry(c as i32).or_insert(0);
        //     })
        //     // line.chars().for_each(|c| {
        //     //     self.plots.entry(c).or_insert((1, 0));
        //     // })
        // });
    }
    fn add_checked(&mut self, x: usize, y: usize) {
        self.checked[x][y] = true;
    }
    fn check_dirs(&mut self, x: usize, y: usize, c: char, id: i32) -> (bool, bool, bool, bool) {
        let uc = if x == 0 { ' ' } else { self.map[x - 1][y] };
        let dc = if x == self.map.len() - 1 {
            ' '
        } else {
            self.map[x + 1][y]
        };
        let lc = if y == 0 { ' ' } else { self.map[x][y - 1] };
        let rc = if y == self.map[x].len() - 1 {
            ' '
        } else {
            self.map[x][y + 1]
        };

        let luc = if x == 0 || y == 0 {
            uc
        } else {
            self.map[x - 1][y - 1]
        };
        let ruc = if x == self.map.len() - 1 || y == 0 {
            uc
        } else {
            self.map[x + 1][y - 1]
        };
        let ldc = if x == 0 || y == self.map[x].len() - 1 {
            dc
        } else {
            self.map[x - 1][y + 1]
        };
        let rdc = if x == self.map.len() - 1 || y == self.map[x].len() - 1 {
            dc
        } else {
            self.map[x + 1][y + 1]
        };

        if !self.checked[x][y] {
            if (luc != c && uc == c && lc == c) || (uc != c && lc != c) {
                *self.sides.entry(id).or_insert(0) += 1;
                println!(
                    "adding sides: {}, id: {}, x: {}, y: {}, luc",
                    self.sides[&id], id, x, y
                );
            }
            if (ruc != c && uc == c && rc == c) || (uc != c && rc != c) {
                *self.sides.entry(id).or_insert(0) += 1;
                println!(
                    "adding sides: {}, id: {}, x: {}, y: {}, ruc",
                    self.sides[&id], id, x, y
                );
            }
            if (ldc != c && lc == c && dc == c) || (lc != c && dc != c) {
                *self.sides.entry(id).or_insert(0) += 1;
                println!(
                    "adding sides: {}, id: {}, x: {}, y: {}, ldc",
                    self.sides[&id], id, x, y
                );
            }
            if (rdc != c && rc == c && dc == c) || (rc != c && dc != c) {
                *self.sides.entry(id).or_insert(0) += 1;
                println!(
                    "adding sides: {}, id: {}, x: {}, y: {}, rdc",
                    self.sides[&id], id, x, y
                );
            }
        }

        let (l, r, u, d) = (lc == c, rc == c, uc == c, dc == c);
        if !l {
            *self.fence.entry(id).or_insert(0) += 1;
        };
        if !r {
            *self.fence.entry(id).or_insert(0) += 1
        };
        if !u {
            *self.fence.entry(id).or_insert(0) += 1
        };
        if !d {
            *self.fence.entry(id).or_insert(0) += 1
        };
        self.add_checked(x, y);
        (l, r, u, d)
    }
    fn check(&mut self, x: usize, y: usize, ls: bool, rs: bool, us: bool, ds: bool, id: i32) {
        if !self.checked[x][y] {
            *self.plots.entry(id).or_insert(0) += 1;
            let c = self.map[x][y];

            let (l, r, u, d) = self.check_dirs(x, y, c, id);

            if l {
                self.check(x, y - 1, l, r, u, d, id);
            }
            if r {
                self.check(x, y + 1, l, r, u, d, id);
            }
            if u {
                self.check(x - 1, y, l, r, u, d, id);
            }
            if d {
                self.check(x + 1, y, l, r, u, d, id);
            }
        };
    }
}

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/day12.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut state = State::new();
    state.add_map(contents);

    let state2 = state.clone();
    let mut ids: Vec<i32> = Vec::new();

    state2.map.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, _c)| {
            // state.checked = state2.checked.clone();
            state.check(x, y, true, true, true, true, x as i32 * 10000 + y as i32);
            ids.push(x as i32 * 10000 + y as i32);
        });
    });
    let mut part1 = 0;
    let mut part2 = 0;
    state.plots.iter().for_each(|(c, f)| {
        if *f > 0 {
            println!("id: {}: \narea: {}", c, f);
            println!("fence:{}", state.fence[&c]);
            part1 += f * state.fence[&c];
            part2 += f * state.sides[&c];
            println!("side: {}\n", state.sides[&c]);
        }
    });
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
