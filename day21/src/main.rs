use std::fs;

fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/day21.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut chain = Chain::new();
    chain.add_robots(2);
    println!("{:?}", chain);

    let mut doors: Vec<String> = Vec::new();
    contents.lines().for_each(|l| {
        doors.push(l.to_string());
    });
    let mut total_score = 0;
    doors.iter().for_each(|d| {
        println!("{}", d);
        let mut score_multiplier = 0;
        let mut moves: Vec<char> = Vec::new();
        d.chars().enumerate().for_each(|(i, c)| {
            if c.is_digit(10) {
                if i == 0 {
                    score_multiplier += c.to_digit(10).unwrap() as u32 * 100;
                } else if i == 1 {
                    score_multiplier += c.to_digit(10).unwrap() as u32 * 10;
                } else if i == 2 {
                    score_multiplier += c.to_digit(10).unwrap() as u32;
                }
            }
            moves.append(&mut chain.get_num_moves(c));
        });
        let mut arrmoves: Vec<char> = Vec::new();
        moves.iter().for_each(|c| {
            arrmoves.append(&mut chain.get_arr_moves(*c, 0));
        });
        let mut arrmoves2: Vec<char> = Vec::new();
        arrmoves.iter().for_each(|c| {
            arrmoves2.append(&mut chain.get_arr_moves(*c, 1));
        });
        let score = arrmoves2.len() as u32;
        total_score += score * score_multiplier;
        println!("score: {}, multiplier: {}", score, score_multiplier);
    });
}

#[derive(Debug)]
struct Chain {
    numpad: Vec<Vec<char>>,
    arrows: Vec<Vec<char>>,
    numpos: (usize, usize),
    robpos: (usize, usize),
    robots: Vec<(usize, usize)>,
}

impl Chain {
    fn new() -> Chain {
        Chain {
            numpad: vec![
                vec!['7', '8', '9'],
                vec!['4', '5', '6'],
                vec!['1', '2', '3'],
                vec!['#', '0', 'A'],
            ],
            arrows: vec![vec!['#', '^', 'A'], vec!['<', 'v', '>']],
            numpos: (3, 2),
            robpos: (0, 2),
            robots: Vec::new(),
        }
    }
    fn add_robots(&mut self, n: usize) {
        for _ in 0..n {
            self.robots.push(self.robpos);
        }
    }
    fn get_num_moves(&mut self, c: char) -> Vec<char> {
        let x = self.numpad.iter().position(|n| n.contains(&c)).unwrap();
        let y = self.numpad[x].iter().position(|n| n == &c).unwrap();
        let (dx, dy) = (
            x as i32 - self.numpos.0 as i32,
            y as i32 - self.numpos.1 as i32,
        );
        let moves = self.num_arrows((dx, dy), self.numpos);
        self.numpos = (x, y);
        moves
    }
    fn num_arrows(&self, moves: (i32, i32), pos: (usize, usize)) -> Vec<char> {
        let mut arrows: Vec<char> = Vec::new();
        if (pos.0 == 3 && pos.1 == 1) || (pos.0 == 2 && pos.1 == 0) {
            if moves.1 > 0 {
                for _ in 0..moves.1 {
                    arrows.push('>');
                }
            } else if moves.1 < 0 {
                for _ in 0..moves.1 * -1 {
                    arrows.push('<');
                }
            }
            if moves.0 < 0 {
                for _ in 0..moves.0 * -1 {
                    arrows.push('^');
                }
            } else if moves.0 > 0 {
                for _ in 0..moves.0 {
                    arrows.push('v');
                }
            }
        } else {
            if moves.0 < 0 {
                for _ in 0..moves.0 * -1 {
                    arrows.push('^');
                }
            } else if moves.0 > 0 {
                for _ in 0..moves.0 {
                    arrows.push('v');
                }
            }
            if moves.1 > 0 {
                for _ in 0..moves.1 {
                    arrows.push('>');
                }
            } else if moves.1 < 0 {
                for _ in 0..moves.1 * -1 {
                    arrows.push('<');
                }
            }
        }
        arrows.push('A');
        // arrows = self.sort_arrows(arrows);
        arrows
    }
    fn get_arr_moves(&mut self, c: char, r: usize) -> Vec<char> {
        let x = self.arrows.iter().position(|n| n.contains(&c)).unwrap();
        let y = self.arrows[x].iter().position(|n| n == &c).unwrap();
        let (dx, dy) = (
            x as i32 - self.robots[r].0 as i32,
            y as i32 - self.robots[r].1 as i32,
        );
        let mut moves = self.return_arrows((dx, dy), self.robots[r]);
        if moves.is_empty() {
            moves.push('A');
        }
        println!("{:?}", moves);
        self.robots[r] = (x, y);
        moves
    }
    fn return_arrows(&self, moves: (i32, i32), pos: (usize, usize)) -> Vec<char> {
        let mut arrows: Vec<char> = Vec::new();
        if pos.0 == 0 && pos.1 == 0 {
            if moves.1 > 0 {
                for _ in 0..moves.1 {
                    arrows.push('>');
                }
            } else if moves.1 < 0 {
                for _ in 0..moves.1 * -1 {
                    arrows.push('<');
                }
            }
            if moves.0 > 0 {
                for _ in 0..moves.0 {
                    arrows.push('v');
                }
            } else if moves.0 < 0 {
                for _ in 0..moves.0 * -1 {
                    arrows.push('^');
                }
            }
        } else {
            if moves.0 > 0 {
                for _ in 0..moves.0 {
                    arrows.push('v');
                }
            } else if moves.0 < 0 {
                for _ in 0..moves.0 * -1 {
                    arrows.push('^');
                }
            }
            if moves.1 > 0 {
                for _ in 0..moves.1 {
                    arrows.push('>');
                }
            } else if moves.1 < 0 {
                for _ in 0..moves.1 * -1 {
                    arrows.push('<');
                }
            }
        }
        arrows.push('A');
        arrows = self.sort_arrows(arrows);
        arrows
    }
    fn sort_arrows(&self, goal: Vec<char>) -> Vec<char> {
        let mut left: Vec<char> = Vec::new();
        let mut right: Vec<char> = Vec::new();
        let mut up: Vec<char> = Vec::new();
        let mut down: Vec<char> = Vec::new();
        let mut end: Vec<char> = Vec::new();
        goal.iter().for_each(|c| {
            match c {
                '^' => up.push(*c),
                'v' => down.push(*c),
                '<' => left.push(*c),
                '>' => right.push(*c),
                'A' => end.push(*c),
                _ => (),
            };
        });
        let first = goal.first().unwrap();
        let mut ret = Vec::new();
        match first {
            '^' => {
                ret.append(&mut up);
                ret.append(&mut down);
                ret.append(&mut left);
                ret.append(&mut right);
                ret.append(&mut end);
            }
            'v' => {
                ret.append(&mut down);
                ret.append(&mut up);
                ret.append(&mut left);
                ret.append(&mut right);
                ret.append(&mut end);
            }
            '<' => {
                ret.append(&mut left);
                ret.append(&mut right);
                ret.append(&mut up);
                ret.append(&mut down);
                ret.append(&mut end);
            }
            '>' => {
                ret.append(&mut right);
                ret.append(&mut left);
                ret.append(&mut up);
                ret.append(&mut down);
                ret.append(&mut end);
            }
            _ => (),
        };
        ret
    }
}
