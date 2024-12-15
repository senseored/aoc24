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
#[derive(Debug, Clone)]
struct BoardWide {
    walls: Vec<(usize, usize)>,
    boxesl: Vec<(usize, usize)>,
    boxesr: Vec<(usize, usize)>,
    robot: (usize, usize),
    moves: String,
    width: usize,
    height: usize,
}
impl BoardWide {
    fn new() -> BoardWide {
        BoardWide {
            walls: Vec::new(),
            boxesl: Vec::new(),
            boxesr: Vec::new(),
            robot: (0, 0),
            moves: String::new(),
            width: 0,
            height: 0,
        }
    }
    fn draw_board(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.walls.contains(&(x, y)) {
                    print!("#");
                } else if self.boxesl.contains(&(x, y)) {
                    print!("[");
                } else if self.boxesr.contains(&(x, y)) {
                    print!("]");
                } else if (x, y) == self.robot {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    fn free_space(&mut self, c: char) -> usize {
        let mut free = 0;
        match c {
            '<' => {
                for x in 1..self.robot.1 {
                    if self.walls.contains(&(self.robot.0, self.robot.1 - x)) {
                        return free;
                    } else if !self.boxesl.contains(&(self.robot.0, self.robot.1 - x))
                        && !self.boxesr.contains(&(self.robot.0, self.robot.1 - x))
                    {
                        free += 1;
                    }
                }
                free
            }
            '>' => {
                for x in self.robot.1 + 1..self.height {
                    if self.walls.contains(&(self.robot.0, x)) {
                        // println!("wall at {}", x);
                        return free;
                    } else if !self.boxesl.contains(&(self.robot.0, x))
                        && !self.boxesr.contains(&(self.robot.0, x))
                    {
                        // println!("free space >");
                        free += 1;
                    }
                }
                // println!("free is {}", free);
                free
            }
            '^' => {
                if self.walls.contains(&(self.robot.0 - 1, self.robot.1)) {
                    return 0;
                } else if !self.boxesl.contains(&(self.robot.0 - 1, self.robot.1))
                    && !self.boxesr.contains(&(self.robot.0 - 1, self.robot.1))
                {
                    return 1;
                } else if self.boxesl.contains(&(self.robot.0 - 1, self.robot.1)) {
                    if self.check_vert(c, self.robot.0 - 1, self.robot.1)
                        && self.check_vert(c, self.robot.0 - 1, self.robot.1 + 1)
                    {
                        free = 1;
                    }
                } else if self.boxesr.contains(&(self.robot.0 - 1, self.robot.1)) {
                    if self.check_vert(c, self.robot.0 - 1, self.robot.1)
                        && self.check_vert(c, self.robot.0 - 1, self.robot.1 - 1)
                    {
                        free = 1;
                    }
                }

                // free = if self.check_vert(c, self.robot.0 - 1, self.robot.1) {
                //     1
                // } else {
                //     0
                // };
                free
            }
            'v' => {
                if self.walls.contains(&(self.robot.0 + 1, self.robot.1)) {
                    return 0;
                } else if !self.boxesl.contains(&(self.robot.0 + 1, self.robot.1))
                    && !self.boxesr.contains(&(self.robot.0 + 1, self.robot.1))
                {
                    return 1;
                } else if self.boxesl.contains(&(self.robot.0 + 1, self.robot.1)) {
                    if self.check_vert(c, self.robot.0 + 1, self.robot.1)
                        && self.check_vert(c, self.robot.0 + 1, self.robot.1 + 1)
                    {
                        free = 1;
                    }
                } else if self.boxesr.contains(&(self.robot.0 + 1, self.robot.1)) {
                    if self.check_vert(c, self.robot.0 + 1, self.robot.1)
                        && self.check_vert(c, self.robot.0 + 1, self.robot.1 - 1)
                    {
                        free = 1;
                    }
                }
                // free = if self.check_vert(c, self.robot.0 + 1, self.robot.1) {
                //     1
                // } else {
                //     0
                // };
                free
            }
            _ => 0,
        }
    }
    fn check_vert(&self, c: char, x: usize, y: usize) -> bool {
        let mut x1 = x;
        match c {
            'v' => {
                x1 += 1;
            }
            '^' => {
                x1 -= 1;
            }
            _ => (),
        }
        if self.walls.contains(&(x1, y)) {
            return false;
        } else if self.boxesl.contains(&(x1, y)) {
            return self.check_vert(c, x1, y) && self.check_vert(c, x1, y + 1);
        } else if self.boxesr.contains(&(x1, y)) {
            return self.check_vert(c, x1, y - 1) && self.check_vert(c, x1, y);
        } else if !self.boxesl.contains(&(x1, y)) && !self.boxesr.contains(&(x1, y)) {
            return true;
        }
        // let y1 = if self.boxesr.contains(&(x, y)) {
        //     y - 1
        // } else {
        //     y
        // };
        // println!("checking {} {} {} {}", x, y, x1, y1);
        // if self.walls.contains(&(x1, y1)) || self.walls.contains(&(x1, y1 + 1)) {
        //     println!("wall at {} {}", x1, y1);
        //     return false;
        // } else if !self.boxesl.contains(&(x1, y)) && !self.boxesr.contains(&(x1, y)) {
        //     return true;
        // }
        // self.check_vert(c, x1, y1) && self.check_vert(c, x1, y1 + 1)
        false
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
                if self.boxesl.contains(&(x, y - 1)) {
                    self.move_boxes(c, x, y - 1);
                    self.boxesl.iter_mut().for_each(|b| {
                        if b.0 == x && b.1 == y - 1 {
                            b.1 -= 1;
                        }
                    });
                }
                if self.boxesr.contains(&(x, y - 1)) {
                    self.move_boxes(c, x, y - 1);
                    self.boxesr.iter_mut().for_each(|b| {
                        if b.0 == x && b.1 == y - 1 {
                            b.1 -= 1;
                        }
                    });
                }
            }
            '>' => {
                if self.boxesl.contains(&(x, y + 1)) {
                    self.move_boxes(c, x, y + 1);
                    self.boxesl.iter_mut().for_each(|b| {
                        if b.0 == x && b.1 == y + 1 {
                            b.1 += 1;
                        }
                    });
                }
                if self.boxesr.contains(&(x, y + 1)) {
                    self.move_boxes(c, x, y + 1);
                    self.boxesr.iter_mut().for_each(|b| {
                        if b.0 == x && b.1 == y + 1 {
                            b.1 += 1;
                        }
                    });
                }
            }
            '^' => {
                if self.boxesl.contains(&(x - 1, y)) {
                    self.move_boxes(c, x - 1, y);
                    self.move_boxes(c, x - 1, y + 1);
                    self.boxesl.iter_mut().for_each(|b| {
                        if b.0 == x - 1 && b.1 == y {
                            b.0 -= 1;
                        }
                    });
                    self.boxesr.iter_mut().for_each(|b| {
                        if b.0 == x - 1 && b.1 == y + 1 {
                            b.0 -= 1;
                        }
                    });
                }
                if self.boxesr.contains(&(x - 1, y)) {
                    self.move_boxes(c, x - 1, y);
                    self.move_boxes(c, x - 1, y - 1);
                    self.boxesl.iter_mut().for_each(|b| {
                        if b.0 == x - 1 && b.1 == y - 1 {
                            b.0 -= 1;
                        }
                    });
                    self.boxesr.iter_mut().for_each(|b| {
                        if b.0 == x - 1 && b.1 == y {
                            b.0 -= 1;
                        }
                    });
                }
            }
            'v' => {
                if self.boxesl.contains(&(x + 1, y)) {
                    self.move_boxes(c, x + 1, y);
                    self.move_boxes(c, x + 1, y + 1);
                    self.boxesl.iter_mut().for_each(|b| {
                        if b.0 == x + 1 && b.1 == y {
                            b.0 += 1;
                        }
                    });
                    self.boxesr.iter_mut().for_each(|b| {
                        if b.0 == x + 1 && b.1 == y + 1 {
                            b.0 += 1;
                        }
                    });
                }
                if self.boxesr.contains(&(x + 1, y)) {
                    self.move_boxes(c, x + 1, y);
                    self.move_boxes(c, x + 1, y - 1);
                    self.boxesl.iter_mut().for_each(|b| {
                        if b.0 == x + 1 && b.1 == y - 1 {
                            b.0 += 1;
                        }
                    });
                    self.boxesr.iter_mut().for_each(|b| {
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
            // println!("{c}");
            // std::io::stdin().read_line(&mut String::new());
        }
    }
    fn calculate_gps(&self) -> i64 {
        let mut total = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.boxesl.contains(&(x, y)) {
                    total += (x as i64 * 100) + (y as i64);
                }
            }
        }
        total
    }
    fn count_boxes(&self) {
        println!("l:{}, r:{}", self.boxesl.len(), self.boxesr.len());
    }
}

fn main() {
    // let file_path = "input/test4.txt";
    // let file_path = "input/test3.txt";
    // let file_path = "input/test2.txt";
    let file_path = "input/test.txt";
    let file_path = "input/day15.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{}", contents);
    // let lines: Vec<&str> = contents.split_whitespace().collect();
    // contents.split_whitespace().for_each(|x| println!("{}", x));

    let mut board = Board::new();
    let mut board2 = BoardWide::new();

    contents.lines().enumerate().for_each(|(x, line)| {
        for (y, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    board.walls.push((x as usize, y as usize));
                    board2.walls.push((x as usize, y * 2 as usize));
                    board2.walls.push((x as usize, y * 2 + 1 as usize));
                }
                '@' => {
                    board.robot = (x as usize, y as usize);
                    board2.robot = (x as usize, y * 2 as usize);
                }
                'O' => {
                    board.boxes.push((x as usize, y as usize));
                    board2.boxesl.push((x as usize, y * 2 as usize));
                    board2.boxesr.push((x as usize, y * 2 + 1 as usize));
                }
                '<' | '>' | '^' | 'v' => {
                    board.moves.push(c);
                    board2.moves.push(c);
                }
                _ => (),
            }
        }
    });
    board.width = board.walls.iter().map(|x| x.1).max().unwrap() + 1;
    board.height = board.walls.iter().map(|x| x.0).max().unwrap() + 1;
    board2.width = board2.walls.iter().map(|x| x.0).max().unwrap() + 1;
    board2.height = board2.walls.iter().map(|x| x.1).max().unwrap() + 1;
    // println!("moves: {}", board.moves);
    board.draw_board();
    board.do_moves();
    println!("gps: {}", board.calculate_gps());
    board2.draw_board();
    board2.count_boxes();
    board2.do_moves();
    board2.draw_board();
    board2.count_boxes();
    println!("gps: {}", board2.calculate_gps());
    //1471436
    //1458740
    // println!("width: {}, height: {}", board2.width, board2.height);
    // board2.draw_board();
    // board2.move_robot('<');
    // board2.draw_board();
    // board2.move_robot('^');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.draw_board();
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.move_robot('>');
    // board2.draw_board();
    // // board2.move_robot('<');
    // // board2.draw_board();
    // // board2.move_robot('<');
    // // board2.draw_board();
    // // board2.move_robot('<');
    // // board2.draw_board();
    // // board2.move_robot('<');
    // // board2.draw_board();
    // // board2.move_robot('<');
    // // board2.draw_board();
    // // board2.move_robot('<');
    // // board2.draw_board();
    // // board.move_robot('>');
    // // board.draw_board();
    // board2.move_robot('v');
    // board2.draw_board();
    // board2.move_robot('^');
    // board2.draw_board();
    // board2.move_robot('v');
    // board2.draw_board();
    // board2.move_robot('<');
    // board2.draw_board();
    // board2.move_robot('^');
    // board2.draw_board();
    // // board.move_robot('<');
    // board.draw_board();
    // board.move_robot('v');
    // board.draw_board();
}
