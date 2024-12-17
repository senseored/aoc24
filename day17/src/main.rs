// use num_traits::pow;
use std::fs;
use std::ops::BitXor;

struct Computer {
    registers: [u64; 3],
    pc: Vec<u64>,
    output: String,
}
impl Computer {
    fn new() -> Self {
        Computer {
            registers: [0, 0, 0],
            pc: Vec::new(),
            output: "".to_string(),
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
}
fn main() {
    let contents = fs::read_to_string("input/test.txt").unwrap();
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
            let mut program = line.split("Program: ").collect::<Vec<&str>>()[1].to_string();
            program = program.split(",").map(|s| s.to_string()).collect();
            program.chars().for_each(|c| {
                computer.pc.push(c as u64 - 0x30);
            });
        }
    });
    println!("{:?}", computer.registers);
    println!("{:?}", computer.pc);

    let mut i = 0;
    while i < computer.pc.len() {
        i = computer.run_command(computer.pc[i], i);
    }

    println!("Part 1: {}", computer.output);
    println!("Part 2: {}", 0);
}
