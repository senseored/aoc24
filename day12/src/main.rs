//want to try message handling to keep one state
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc;

#[derive(Clone)]
struct State {
    map: Vec<Vec<char>>,
    plots: HashMap<char, i32>,
    fence: HashMap<char, i32>,
    sides: HashMap<char, i32>,
    checked: Vec<Vec<bool>>,
    schecked: Vec<Vec<bool>>,
}

impl State {
    fn new() -> State {
        State {
            map: Vec::new(),
            plots: HashMap::new(),
            fence: HashMap::new(),
            sides: HashMap::new(),
            checked: Vec::new(),
            schecked: Vec::new(),
        }
    }
    fn add_map(&mut self, contents: String) {
        self.map = contents
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        self.checked = vec![vec![false; self.map[0].len()]; self.map.len()];
        self.schecked = vec![vec![false; self.map[0].len()]; self.map.len()];
        contents.lines().for_each(|line| {
            line.chars().for_each(|c| {
                *self.plots.entry(c).or_insert(0) += 1;
                self.fence.entry(c).or_insert(0);
                self.sides.entry(c).or_insert(0);
            })
            // line.chars().for_each(|c| {
            //     self.plots.entry(c).or_insert((1, 0));
            // })
        });
        // self.plots.iter().for_each(|(c, x)| {
        //     println!("{}: {}", c, x);
        // });
    }
    fn add_schecked(&mut self, x: usize, y: usize) {
        self.schecked[x][y] = true;
    }
    fn add_checked(&mut self, x: usize, y: usize) {
        self.checked[x][y] = true;
    }
    fn check_dirs(&mut self, x: usize, y: usize, c: char) -> (bool, bool, bool, bool) {
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
        let (l, r, u, d) = (lc == c, rc == c, uc == c, dc == c);
        if !l {
            *self.fence.entry(c).or_insert(0) += 1
        };
        if !r {
            *self.fence.entry(c).or_insert(0) += 1
        };
        if !u {
            *self.fence.entry(c).or_insert(0) += 1
        };
        if !d {
            *self.fence.entry(c).or_insert(0) += 1
        };
        (l, r, u, d)
    }
    fn check(&mut self, x: usize, y: usize, ls: bool, rs: bool, us: bool, ds: bool) {
        self.add_checked(x, y);
        let c = self.map[x][y];
        // println!("{}: x:{},y:{}", c, x, y);

        let (l, r, u, d) = self.check_dirs(x, y, c);
        if !self.schecked[x][y] {
            if ls && !l {
                *self.sides.entry(c).or_insert(0) += 1;
            }
            if rs && !r {
                *self.sides.entry(c).or_insert(0) += 1;
            }
            if us && !u {
                *self.sides.entry(c).or_insert(0) += 1;
            }
            if ds && !d {
                *self.sides.entry(c).or_insert(0) += 1;
            }
        }
        self.add_schecked(x, y);
        if l {
            if !self.checked[x][y - 1] {
                self.check(x, y - 1, l, r, u, d);
            }
        }
        if r {
            if !self.checked[x][y + 1] {
                self.check(x, y + 1, l, r, u, d);
            }
        }
        if u {
            if !self.checked[x - 1][y] {
                self.check(x - 1, y, l, r, u, d);
            }
        }
        if d {
            if !self.checked[x + 1][y] {
                self.check(x + 1, y, l, r, u, d);
            }
        }
        // println!("{}: fence:{}, plots:{}", c, self.fence[&c], self.plots[&c]);
    }
}

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/day12.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");
    let mut state = State::new();
    state.add_map(contents);

    let state2 = state.clone();

    state2.map.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, _c)| {
            state.checked = state2.checked.clone();
            state.check(x, y, true, true, true, true);
        });
    });

    let (mut count, mut sides, mut cost) = (0, 0, 0);

    for (key, val) in state.plots.iter() {
        println!("{}: {}", key, val);
        println!("fence: {}", state.fence[&key]);
        println!("side: {}", state.sides[&key]);
        println!();
    }

    state.fence.iter().for_each(|(c, f)| {
        count += *f as i32;
        sides += *f as i32 * state.sides[&c] as i32;
        cost += *f as i32 * state.plots[&c] as i32;
        println!("{} - {} * {}", c, state.sides[&c], state.plots[&c]);
    });
    // state.sides.iter().for_each(|(c, f)| {
    //     sides += *f as i32;
    //     cost += *f as i32 * state.plots[&c] as i32;
    //     println!("{} - {} * {}", c, *f, state.plots[&c]);
    // });
    println!("count: {count}");
    println!("sides: {sides}");
    println!("cost: {cost}");
    // println!("{:?}", state.check(0, 0));
    // println!("{:?}", state.check(1, 0));
    // println!("{:?}", state.check(3, 0));
    // println!("{:?}", state.check(2, 2));
    // println!("{:?}", state.check(4, 2));

    // println!("{:?}", state.check(1, 1));
    // println!("{:?}", state.check(1, 3));
    // println!("{:?}", state.check(3, 1));
    // println!("{:?}", state.check(3, 3));

    // let (tx, rx) = mpsc::channel();
    //
    // for recieved in rx -> State {
    //     println!("recieved: {recieved}");
    //     state
    // }
}
