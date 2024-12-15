use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Board {
    walls: Vec<(usize, usize)>,
    boxes: Vec<(usize, usize)>,
    robot: (usize, usize),
    moves: String,
    width: usize,
    height: usize,
}
impl Board {
    fn new() -> Board {
        Board {
            walls: Vec::new(),
            boxes: Vec::new(),
            robot: (0, 0),
            moves: String::new(),
            width: 0,
            height: 0,
        }
    }
    fn free_space(&mut self, c: char) -> usize {
        let mut free = 0;
        match c {
            '<' => {
                for x in 1..self.robot.1 {
                    if self.walls.contains(&(self.robot.0, self.robot.1 - x)) {
                        return free;
                    } else if !self.boxes.contains(&(self.robot.0, self.robot.1 - x)) {
                        free += 1;
                    }
                }
                free
            }
            '>' => {
                for x in self.robot.1 + 1..self.width {
                    if self.walls.contains(&(self.robot.0, x)) {
                        return free;
                    } else if !self.boxes.contains(&(self.robot.0, x)) {
                        free += 1;
                    }
                }
                free
            }
            '^' => {
                for y in 1..self.robot.0 {
                    if self.walls.contains(&(self.robot.0 - y, self.robot.1)) {
                        return free;
                    } else if !self.boxes.contains(&(self.robot.0 - y, self.robot.1)) {
                        free += 1;
                    }
                }
                free
            }
            'v' => {
                for y in self.robot.0 + 1..self.height {
                    if self.walls.contains(&(y, self.robot.1)) {
                        return free;
                    } else if !self.boxes.contains(&(y, self.robot.1)) {
                        free += 1;
                    }
                }
                free
            }
            _ => 0,
        }
    }
    fn move_robot(&mut self, c: char) {
        match c {
            '<' => {
                self.robot.1 -= if self.free_space(c) > 0 {
                    self.move_boxes(c, self.robot.0, self.robot.1);
                    1
                } else {
                    0
                };
            }
            '>' => {
                self.robot.1 += if self.free_space(c) > 0 {
                    self.move_boxes(c, self.robot.0, self.robot.1);
                    1
                } else {
                    0
                };
            }
            '^' => {
                self.robot.0 -= if self.free_space(c) > 0 {
                    self.move_boxes(c, self.robot.0, self.robot.1);
                    1
                } else {
                    0
                };
            }
            'v' => {
                self.robot.0 += if self.free_space(c) > 0 {
                    self.move_boxes(c, self.robot.0, self.robot.1);
                    1
                } else {
                    0
                };
            }
            _ => (),
        }
    }
    fn move_boxes(&mut self, c: char, x: usize, y: usize) {
        match c {
            '<' => {
                if self.boxes.contains(&(x, y - 1)) {
                    self.move_boxes(c, x, y - 1);
                    self.boxes.iter_mut().for_each(|b| {
                        if b.0 == x && b.1 == y - 1 {
                            b.1 -= 1;
                        }
                    });
                }
            }
            '>' => {
                if self.boxes.contains(&(x, y + 1)) {
                    self.move_boxes(c, x, y + 1);
                    self.boxes.iter_mut().for_each(|b| {
                        if b.0 == x && b.1 == y + 1 {
                            b.1 += 1;
                        }
                    });
                }
            }
            '^' => {
                if self.boxes.contains(&(x - 1, y)) {
                    self.move_boxes(c, x - 1, y);
                    self.boxes.iter_mut().for_each(|b| {
                        if b.0 == x - 1 && b.1 == y {
                            b.0 -= 1;
                        }
                    });
                }
            }
            'v' => {
                if self.boxes.contains(&(x + 1, y)) {
                    self.move_boxes(c, x + 1, y);
                    self.boxes.iter_mut().for_each(|b| {
                        if b.0 == x + 1 && b.1 == y {
                            b.0 += 1;
                        }
                    });
                }
            }
            _ => (),
        }
    }
    fn do_moves(&mut self) {
        let moves: Vec<char> = self.moves.chars().collect();
        for c in moves {
            self.move_robot(c);
            // self.draw_board();
        }
    }
    fn draw_board(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.walls.contains(&(x, y)) {
                    print!("#");
                } else if self.boxes.contains(&(x, y)) {
                    print!("O");
                } else if (x, y) == self.robot {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    fn calculate_gps(&self) -> i64 {
        let mut total = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.boxes.contains(&(x, y)) {
                    total += (x as i64 * 100) + (y as i64);
                }
            }
        }
        total
    }
}

fn main() {
    let file_path = "input/test2.txt";
    let file_path = "input/test.txt";
    let file_path = "input/day15.txt";

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
                '@' => {
                    board.robot = (x as usize, y as usize);
                }
                'O' => {
                    board.boxes.push((x as usize, y as usize));
                }
                '<' | '>' | '^' | 'v' => {
                    board.moves.push(c);
                }
                _ => (),
            }
        }
    });
    board.width = board.walls.iter().map(|x| x.1).max().unwrap() + 1;
    board.height = board.walls.iter().map(|x| x.0).max().unwrap() + 1;
    // println!("moves: {}", board.moves);
    board.do_moves();
    println!("gps: {}", board.calculate_gps());
    // board.draw_board();
    // board.move_robot('>');
    // board.draw_board();
    // board.move_robot('>');
    // board.draw_board();
    // board.move_robot('v');
    // board.draw_board();
    // board.move_robot('^');
    // board.draw_board();
    // board.move_robot('<');
    // board.draw_board();
    // board.move_robot('v');
    // board.draw_board();
}
