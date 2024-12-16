use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Board {
    walls: Vec<(usize, usize)>,
    boxes: Vec<(usize, usize)>,
    horse: (usize, usize),
    end: (usize, usize),
    width: usize,
    height: usize,
}
impl Board {
    fn new() -> Board {
        Board {
            walls: Vec::new(),
            boxes: Vec::new(),
            horse: (0, 0),
            end: (0, 0),
            width: 0,
            height: 0,
        }
    }
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
}
fn main() {
    let file_path = "input/test.txt";
    // let file_path = "input/day16.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // println!("{}", contents);
    // let lines: Vec<&str> = contents.split_whitespace().collect();
    // contents.split_whitespace().for_each(|x| println!("{}", x));

    let mut board = Board::new();

    contents.lines().enumerate().for_each(|(y, line)| {
        for (x, c) in line.chars().enumerate() {
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
    board.draw_board();
}

