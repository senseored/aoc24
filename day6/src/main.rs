use std::fs;

#[derive(Clone)]
struct Area {
    // area: [[char; 130]; 130],
    area: Vec<Vec<char>>,
    loops: i32,
}
impl Area {
    fn new() -> Area {
        Area {
            area: Vec::new(),
            // area: [['.'; 130]; 130],
            loops: 0,
        }
    }
    fn add_loop(&mut self) {
        self.loops += 1;
    }
    fn print_loops(&self) {
        println!("loops: {}", self.loops);
    }
    fn populate(&mut self, contents: &str) {
        contents.lines().for_each(|lines| {
            let mut line = Vec::new();
            lines.chars().for_each(|char| {
                line.push(char);
            });
            self.area.push(line);
        });
        // for lines in contents.lines() {
        //     let mut line = Vec::new();
        //     for char in lines.chars() {
        //         line.push(char);
        //     }
        //     self.area.push(line);
        // }
    }
    fn print(&self) {
        self.area.clone().into_iter().for_each(|line| {
            println!("{:?}", line);
        });
        // for i in 0..self.area.len() {
        //     println!("{:?}", self.area[i]);
        // }
    }
    fn get_position(&self) -> (usize, usize, char) {
        for y in 0..self.area.len() {
            for x in 0..self.area[y].len() {
                match self.area[y][x] {
                    '<' | '>' | '^' | 'v' => {
                        return (x, y, self.area[y][x]);
                    }
                    _ => {}
                }
            }
        }
        (0, 0, ' ')
    }
    fn get_width(&self) -> usize {
        println!("width: {}", self.area[0].len());
        self.area[0].len()
    }
    fn get_height(&self) -> usize {
        println!("height: {}", self.area.len());
        self.area.len()
    }
    fn do_moves(&mut self, mut x: usize, mut y: usize, mut char: char) -> i32 {
        while !(x == 0 || x == self.area[0].len() - 1 || y == 0 || y == self.area.len() - 1) {
            (x, y, char) = self.move_char(x, y, char);
            // self.print();
        }
        self.area[y][x] = 'X';
        self.calculate_x()
    }
    fn calculate_x(&self) -> i32 {
        let mut result = 0;
        for y in 0..self.area.len() {
            for x in 0..self.area[y].len() {
                if self.area[y][x] == 'X' {
                    result += 1;
                }
            }
        }
        println!("result: {}", result);
        println!("loops: {}", self.loops);
        result
    }
    fn move_char(&mut self, mut x: usize, mut y: usize, char: char) -> (usize, usize, char) {
        // area: Vec<Vec<char>>,
        let mut looparea: Box<Vec<Vec<char>>> = Box::new(Vec::new());
        looparea = Box::new(self.area.clone());
        self.print_loops();
        match char {
            '<' => {
                if self.area[y][x - 1] == '#' {
                    self.area[y][x] = '^';
                } else {
                    looparea[y][x - 1] = '#';
                    self.check_loops(looparea, x, y, x, y, '<', true);
                    self.area[y][x] = 'X';
                    x -= 1;
                    self.area[y][x] = '<';
                }
            }
            '>' => {
                if self.area[y][x + 1] == '#' {
                    self.area[y][x] = 'v';
                } else {
                    looparea[y][x + 1] = '#';
                    self.check_loops(looparea, x, y, x, y, '>', true);
                    self.area[y][x] = 'X';
                    x += 1;
                    self.area[y][x] = '>';
                }
            }
            '^' => {
                if self.area[y - 1][x] == '#' {
                    self.area[y][x] = '>';
                } else {
                    looparea[y - 1][x] = '#';
                    self.check_loops(looparea, x, y, x, y, '^', true);
                    self.area[y][x] = 'X';
                    y -= 1;
                    self.area[y][x] = '^';
                }
            }
            'v' => {
                if self.area[y + 1][x] == '#' {
                    self.area[y][x] = '<';
                } else {
                    looparea[y + 1][x] = '#';
                    self.check_loops(looparea, x, y, x, y, 'v', true);
                    self.area[y][x] = 'X';
                    y += 1;
                    self.area[y][x] = 'v';
                }
            }
            _ => {}
        }
        (x, y, self.area[y][x])
    }
    fn check_loops(
        &mut self,
        area: Box<Vec<Vec<char>>>,
        x: usize,
        y: usize,
        start_x: usize,
        start_y: usize,
        char: char,
        firstloop: bool,
    ) {
        if x == start_x && y == start_y && !firstloop {
            self.add_loop();
        } else if x > 0 && x < area[0].len() - 1 && y > 0 && y < area.len() - 1 {
            match char {
                '<' => {
                    if area[y][x - 1] == '#' {
                        self.check_loops(area, x, y, start_x, start_y, '^', firstloop);
                        // area[y][x] = '^';
                    } else {
                        self.check_loops(area, x - 1, y, start_x, start_y, '<', false);
                        // area[y][x] = 'X';
                        // x -= 1;
                        // area[y][x] = '<';
                    }
                }
                '>' => {
                    if area[y][x + 1] == '#' {
                        self.check_loops(area, x, y, start_x, start_y, 'v', firstloop);
                        // area[y][x] = 'v';
                    } else {
                        self.check_loops(area, x + 1, y, start_x, start_y, '>', false);
                        // area[y][x] = 'X';
                        // x += 1;
                        // area[y][x] = '>';
                    }
                }
                '^' => {
                    if area[y - 1][x] == '#' {
                        self.check_loops(area, x, y, start_x, start_y, '>', firstloop);
                        // area[y][x] = '>';
                    } else {
                        self.check_loops(area, x, y - 1, start_x, start_y, '^', false);
                        // area[y][x] = 'X';
                        // y -= 1;
                        // area[y][x] = '^';
                    }
                }
                'v' => {
                    if area[y + 1][x] == '#' {
                        self.check_loops(area, x, y, start_x, start_y, '<', firstloop);
                        // area[y][x] = '<';
                    } else {
                        // area[y][x] = 'X';
                        // area[y][x] = 'v';
                        self.check_loops(area, x, y + 1, start_x, start_y, 'v', false);
                    }
                }
                _ => {}
            }
        }
    }
}
// impl Drop for Area {
//     fn drop(&mut self) {
//         let mut children = mem::replace(&mut self.area, Vec::new());
//
//         loop {
//             children = match children {
//                 Vec(mut n) => mem::replace(&mut n.borrow_mut().area, Vec::new()),
//                 // None => break,
//             }
//         }
//     }
// }
fn main() {
    // let file_path = "input/test.txt";
    let file_path = "input/day6.txt";

    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{contents}");

    let mut area = Box::new(Area::new()); //Area::new();
    area.populate(&contents);
    // area.print();
    let (x, y, char) = area.get_position();
    println!("x: {}, y: {}", x, y);
    area.get_width();
    area.get_height();

    area.do_moves(x, y, char);
}
