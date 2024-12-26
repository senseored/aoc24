use std::fs;

#[derive(Debug)]
struct Chain {
    numpad: Vec<Vec<char>>,
    arrows: Vec<Vec<char>>,
    numpos: (i32, i32),
    r1pos: (i32, i32),
    r2pos: (i32, i32),
}

impl Chain {
    fn new() -> Chain {
        Chain {
            numpad: vec![
                vec!['7', '8', '9'],
                vec!['4', '5', '6'],
                vec!['1', '2', '3'],
                vec!['_', '0', 'A'],
            ],
            arrows: vec![vec!['_', '^', 'A'], vec!['<', 'v', '>']],
            numpos: (3, 2),
            r1pos: (0, 2),
            r2pos: (0, 2),
        }
    }
    fn reset_pos(&mut self) {
        self.numpos = (3, 2);
        self.r1pos = (0, 2);
        self.r2pos = (0, 2);
    }
    fn get_arrow_pos(&self, pos: (i32, i32)) -> char {
        self.arrows[pos.0 as usize][pos.1 as usize]
    }

    fn new_pos(&self, pos: (i32, i32), c: char) -> (i32, i32) {
        match c {
            '^' => (pos.0 - 1, pos.1),
            'v' => (pos.0 + 1, pos.1),
            '<' => (pos.0, pos.1 - 1),
            '>' => (pos.0, pos.1 + 1),
            _ => pos,
        }
    }

    fn fix_moves(&mut self, moves: Vec<char>) -> Vec<char> {
        let mut new_moves: Vec<char> = Vec::new();
        moves.iter().for_each(|m| {
            match m {
                '^' => {
                    self.r1pos = self.new_pos(self.r1pos, *m);
                    new_moves.push('^');
                }
                'v' => {
                    self.r1pos = self.new_pos(self.r1pos, *m);
                    new_moves.push('v');
                }
                '<' => {
                    self.r1pos = self.new_pos(self.r1pos, *m);
                    new_moves.push('<');
                }
                '>' => {
                    self.r1pos = self.new_pos(self.r1pos, *m);
                    new_moves.push('>');
                }
                'A' => {
                    // println!("moving r1");
                    self.r1pos = self.new_pos(self.r1pos, *m);
                    self.r2pos = self.new_pos(self.r2pos, self.get_arrow_pos(self.r1pos));
                    if self.get_arrow_pos(self.r1pos) == 'A' {
                        self.numpos = self.new_pos(self.numpos, self.get_arrow_pos(self.r2pos));
                        // if self.get_arrow_pos(self.r2pos) == 'A' {
                        //     print!(
                        //         "{}",
                        //         self.numpad[self.numpos.0 as usize][self.numpos.1 as usize]
                        //     );
                        // }
                        // print!("{}", self.get_arrow_pos(self.r2pos));
                    }
                    // print!("{}", self.get_arrow_pos(self.r1pos));
                    // println!("moving r2");
                    // println!("moving num");
                    // self.numpos = self.new_pos(self.numpos, self.get_arrow_pos(self.r2pos));
                    // if self.get_arrow_pos(self.r2pos) == 'A' {
                    //     println!(
                    //         "{}",
                    //         self.numpad[self.numpos.0 as usize][self.numpos.1 as usize]
                    //     );
                    // }
                    if self.numpos == (3, 0) {
                        println!("previous bad move: {}", new_moves[new_moves.len() - 1]);
                        if new_moves[new_moves.len() - 1] == 'A' {
                            new_moves.push('x');
                        }
                        // // println!("previous bad move: {}", new_moves[new_moves.len()]);
                        new_moves.pop();
                        new_moves.push('x');
                        new_moves.push('x');
                        println!("faulty move sucker");
                    }
                    new_moves.push('A');
                }
                _ => (),
            };
        });
        new_moves
    }
}

fn main() {
    let file_path = "input/test.txt";
    let file_path = "input/day21.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut doors: Vec<String> = Vec::new();
    contents.lines().for_each(|l| {
        doors.push(l.to_string());
    });

    let mut chain = Chain::new();

    let mut total_score = 0;
    doors.iter().for_each(|d| {
        println!("door: {}", d);
        let mut totmoves: Vec<char> = Vec::new();
        let mut score_multiplier = 0;
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
        });
        let mut score = 0;
        let moves = get_num_moves(&chain.numpad, d.to_string());
        // println!("moves: {:?}", moves);
        chain.reset_pos();
        moves.iter().for_each(|m| {
            let arr_moves = get_arr_moves(&chain.arrows, m.to_vec());
            // println!("moves: {:?}", arr_moves);
            arr_moves.iter().for_each(|a| {
                let arr_moves2 = get_arr_moves(&chain.arrows, a.to_vec());
                // println!("moves: {:?}", arr_moves2);
                arr_moves2.iter().for_each(|m| {
                    totmoves.append(&mut m.to_vec());
                    // let v = chain.fix_moves(m.to_vec());
                    // m.iter().for_each(|c| {
                    //     print!("{}", c);
                    // });
                    // print!("A");
                    // score += v.len() as u32;
                    // score += m.len() as u32 + 1;
                });
            });
        });
        let v = chain.fix_moves(totmoves);
        // println!("moves: {:?}", v);
        score += v.len() as u32;
        // score += totmoves.len() as u32;
        total_score += score * score_multiplier;
        println!("score: {}, multiplier: {}", score, score_multiplier);
    });
    println!("total score: {}", total_score);
}

fn get_num_moves(numpad: &Vec<Vec<char>>, goal: String) -> Vec<Vec<char>> {
    let mut moves: Vec<Vec<char>> = Vec::new();
    let mut pos = (3, 2);
    goal.chars().for_each(|c| {
        let x = numpad.iter().position(|n| n.contains(&c)).unwrap();
        let y = numpad[x].iter().position(|n| n == &c).unwrap();
        let (dx, dy) = (x as i32 - pos.0 as i32, y as i32 - pos.1 as i32);
        moves.push(num_arrows((dx, dy), pos));
        pos = (x, y);
    });
    moves
}

fn get_arr_moves(arrows: &Vec<Vec<char>>, mut goal: Vec<char>) -> Vec<Vec<char>> {
    goal = sort_arrows(goal);
    println!("goal: {:?}", goal);
    // goal = sort_num_arrows(goal);
    // println!("goal: {:?}", goal);
    let mut moves: Vec<Vec<char>> = Vec::new();
    let mut pos = (0, 2);
    goal.iter().for_each(|c| {
        let x = arrows.iter().position(|d| d.contains(&c)).unwrap();
        let y = arrows[x].iter().position(|d| d == c).unwrap();

        let (dx, dy) = (x as i32 - pos.0 as i32, y as i32 - pos.1 as i32);
        moves.push(return_arrows((dx, dy), pos));
        pos = (x, y);
    });
    // let (x, y) = (0, 2);
    // let (dx, dy) = (x as i32 - pos.0 as i32, y as i32 - pos.1 as i32);
    // moves.push(return_arrows((dx, dy)));
    moves
}

fn sort_arrows(goal: Vec<char>) -> Vec<char> {
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
    right.append(&mut down);
    right.append(&mut left);
    right.append(&mut up);
    right.append(&mut end);
    right
}

fn sort_num_arrows(goal: Vec<char>) -> Vec<char> {
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
    up.append(&mut right);
    up.append(&mut down);
    up.append(&mut left);
    up.append(&mut end);
    up
}

// fn new_num_pos(pos: (i32, i32), c: char) -> (i32, i32) {
//     match c {
//         '^' => (pos.0 - 1, pos.1),
//         'v' => (pos.0 + 1, pos.1),
//         '<' => (pos.0, pos.1 - 1),
//         '>' => (pos.0, pos.1 + 1),
//         _ => pos,
//     }
// }
//
// fn sort_num_arrows(moves: Vec<char>) -> Vec<char> {
//     let mut pos = (3, 2);
//     let mut new_moves: Vec<char> = Vec::new();
//     moves.iter().for_each(|c| {
//         let mut new_pos = new_num_pos(pos, *c);
//         if new_pos != (3, 0) {
//             new_moves.push(*c);
//             print!("{}", c);
//         } else {
//             print!("     ");
//             match c {
//                 'v' => {
//                     new_moves.push('>');
//                     new_moves.push('v');
//                     new_pos = (3, 1);
//                 }
//                 '<' => {
//                     new_moves.push('^');
//                     new_moves.push('<');
//                     new_pos = (2, 0);
//                 }
//                 _ => (),
//             }
//         }
//         pos = new_pos;
//     });
//     new_moves
// }

fn num_arrows(moves: (i32, i32), pos: (usize, usize)) -> Vec<char> {
    let mut arrows: Vec<char> = Vec::new();
    if pos.0 == 2 && pos.1 == 0 {
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
    // arrows = sort_arrows(arrows);
    // arrows = sort_num_arrows(arrows);
    arrows
}

fn return_arrows(moves: (i32, i32), pos: (usize, usize)) -> Vec<char> {
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
    // arrows = sort_arrows(arrows);
    arrows
}
