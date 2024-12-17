// use num_traits::pow;
use std::fs;
use std::ops::BitXor;

struct Computer {
    registers: [u64; 3],
    pc: Vec<u64>,
    output: String,
    start: String,
}
impl Computer {
    fn new() -> Self {
        Computer {
            registers: [0, 0, 0],
            pc: Vec::new(),
            output: "".to_string(),
            start: String::new(),
        }
    }
    fn adv(&mut self, i: usize) -> usize {
        // println!("adv, i: {}", i);
        let a = self.registers[0];
        let b = if self.pc[i + 1] <= 3 {
            2u64.pow(self.pc[i + 1].try_into().unwrap())
        } else {
            2u64.pow(
                self.registers[self.pc[i + 1] as usize - 4]
                    .try_into()
                    .unwrap(),
            )
        };
        let f = (a as f64 / b as f64) as f64;
        self.registers[0] = f.trunc() as u64;
        i + 2
    }
    fn bxl(&mut self, i: usize) -> usize {
        // println!("bxl, i: {}", i);
        let a = self.registers[1];
        let b = self.pc[i + 1];
        self.registers[1] = BitXor::bitxor(a, b);
        i + 2
    }
    fn bst(&mut self, i: usize) -> usize {
        // println!("bst, i: {}", i);
        let b = if self.pc[i + 1] <= 3 {
            self.pc[i + 1]
        } else {
            self.registers[self.pc[i + 1] as usize - 4]
        };
        self.registers[1] = b % 8;
        i + 2
    }
    fn jnz(&mut self, i: usize) -> usize {
        // println!("jnz, i: {}", i);
        let mut ret = i + 2;
        if self.registers[0] > 0 {
            ret = self.pc[i + 1] as usize;
        }
        ret
    }
    fn bxc(&mut self, i: usize) -> usize {
        // println!("bxc, i: {}", i);
        let a = self.registers[1];
        let b = self.registers[2];
        self.registers[1] = BitXor::bitxor(a, b);
        i + 2
    }
    fn out(&mut self, i: usize) -> usize {
        // println!("out, i: {}", i);
        let b = if self.pc[i + 1] <= 3 {
            self.pc[i + 1]
        } else {
            self.registers[self.pc[i + 1] as usize - 4]
        };
        if self.output == "" {
            self.output = format!("{}", b % 8);
        } else {
            self.output = format!("{},{}", self.output, b % 8);
        }
        i + 2
    }
    fn bdv(&mut self, i: usize) -> usize {
        // println!("bdv, i: {}", i);
        let a = self.registers[0];
        let b = if self.pc[i + 1] <= 3 {
            2u64.pow(self.pc[i + 1].try_into().unwrap())
        } else {
            2u64.pow(
                self.registers[self.pc[i + 1] as usize - 4]
                    .try_into()
                    .unwrap(),
            )
        };
        let f = (a as f64 / b as f64) as f64;
        self.registers[1] = f.trunc() as u64;
        i + 2
    }
    fn cdv(&mut self, i: usize) -> usize {
        // println!("cdv, i: {}", i);
        let a = self.registers[0];
        let b = if self.pc[i + 1] <= 3 {
            2u64.pow(self.pc[i + 1].try_into().unwrap())
        } else {
            2u64.pow(
                self.registers[self.pc[i + 1] as usize - 4]
                    .try_into()
                    .unwrap(),
            )
        };
        let f = (a as f64 / b as f64) as f64;
        self.registers[2] = f.trunc() as u64;
        i + 2
    }
    fn run_command(&mut self, c: u64, i: usize) -> usize {
        match c {
            0 => self.adv(i),
            1 => self.bxl(i),
            2 => self.bst(i),
            3 => self.jnz(i),
            4 => self.bxc(i),
            5 => self.out(i),
            6 => self.bdv(i),
            7 => self.cdv(i),
            _ => panic!("Unknown command: {}", c),
        }
    }
    fn print_computer(&self) {
        println!("registers: {:?}", self.registers);
        println!("pc: {:?}", self.pc);
        println!("start: {}", self.start);
        println!("output: {}", self.output);
    }
}
fn main() {
    let contents = fs::read_to_string("input/test.txt").unwrap();
    let contents = fs::read_to_string("input/test2.txt").unwrap();
    let contents = fs::read_to_string("input/day17.txt").unwrap();
    println!("{}", contents);

    let mut computer = Computer::new();

    contents.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            computer.registers[0] = line.split("Register A: ").collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();
        }
        if i == 1 {
            computer.registers[1] = line.split("Register B: ").collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();
        }
        if i == 2 {
            computer.registers[2] = line.split("Register C: ").collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();
        }
        if i == 4 {
            computer.start = line.split("Program: ").collect::<Vec<&str>>()[1].to_string();
            let program: String = computer.start.split(",").map(|s| s.to_string()).collect();
            program.chars().for_each(|c| {
                computer.pc.push(c as u64 - 0x30);
            });
        }
    });
    println!("{:?}", computer.registers);
    println!("{:?}", computer.pc);
    println!("{}", computer.start);

    let mut i = 0;
    while i < computer.pc.len() {
        i = computer.run_command(computer.pc[i], i);
    }

    println!("Part 1: {}", computer.output);

    computer.print_computer();
    // let mut x: i64 = 100000000000000;
    // let mut x: i64 = 4398046500000;
    // let mut x: i64 = 4398046499990;
    // let mut x: i64 = 1135902984;
    // let mut x: i64 = 9087223872;
    // let mut x: i64 = 72697793112;
    // let mut x: i64 = 581582344944;
    // let mut x: u64 = 4652658759568;
    // let mut x: u64 = 37221270076899;
    let mut x: u64 = 37221270076544;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 =          ;
    // let mut x: i64 = 0;
    computer.output = "".to_string();
    let mut res: Vec<String> = Vec::new();
    res.push(computer.start.clone());

    computer.print_computer();
    for i in 1..computer.start.len() {
        res.push(rem_first_and_last(&res[i - 1]).to_string());
    }
    computer.registers = [0, 0, 0];
    res = res.into_iter().rev().collect();
    println!("{:?}", res);

    // let n: i32 = -1;
    // for i in 0..32 {
    //     (0..32)
    //         .map(|n| x - 1 >> n)
    //         .for_each(|n| print!("{}", if n % 2 == 0 { "0" } else { "1" }));
    //     println!(" - result: {}", n);
    // }

    for y in 0..res.len() {
        for i in 0..y {
            if i % 2 == 0 {
                while !computer.output.ends_with(res[i].as_str()) {
                    computer.registers = [x.try_into().unwrap(), 0, 0];
                    computer.output = "".to_string();
                    let mut i = 0;
                    while i < computer.pc.len() {
                        i = computer.run_command(computer.pc[i], i);
                    }
                    x += 1;
                }
                // println!("result: {}, res: {}", x - 1, res[i]);
            }
        }
        if y % 2 == 0 {
            (0..64)
                .map(|n| x - 1 >> n)
                .for_each(|n| print!("{}", if n % 2 == 0 { "0" } else { "1" }));
            println!(" - result: {}, res: {}", x - 1, res[y]);
        }
    }
    // let mut x: i64 = 35184372089000;

    // while computer.output != computer.start {
    //     // while computer.output != "3,0" {
    //     // while !computer.output.ends_with("5,0,3,3,0") {
    //     if x % 10000 == 0 {
    //         println!("x: {}", x);
    //     }
    //     computer.registers = [x.try_into().unwrap(), 0, 0];
    //     computer.output = "".to_string();
    //     let mut i = 0;
    //     while i < computer.pc.len() {
    //         i = computer.run_command(computer.pc[i], i);
    //     }
    //     x += 1;
    // }
    // println!("result: {}, output: {}", x - 1, computer.output);

    // let program: String = computer.start.split(",").map(|s| s.to_string()).collect();
    // program.chars().for_each(|c| {
    //     computer.pc.push(c as u64 - 0x30);
    // });

    // let mut res: Vec<String> = Vec::new();
    // for i in 0..computer.pc.len() {
    //     res.push(computer.pc[computer.pc.len() - 1].to_string());
    //     // res.push("".to_string());
    //     for x in 0..i {
    //         res[x] = format!("{}{}", res[x], computer.pc[computer.pc.len() - i - 1]);
    //     }
    //     // res.push(computer.pc[computer.pc.len() - i - 1].to_string());
    // }
    println!("{:?}", res);
    println!("Part 2: {}", 0);

    computer.registers = [x - 1, 0, 0];
    computer.output = "".to_string();

    res.push(computer.start.clone());

    println!("Part 1: {}", computer.output);
    println!("{:?}", computer.registers);
    println!("{:?}", computer.pc);
    println!("{}", computer.start);

    let mut i = 0;
    while i < computer.pc.len() {
        i = computer.run_command(computer.pc[i], i);
    }

    computer.print_computer();
}
fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    // chars.next_back();
    chars.as_str()
}
