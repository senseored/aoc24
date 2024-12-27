use std::fs;

fn main() {
    let contents = fs::read_to_string("input/test.txt").unwrap();
    let contents = fs::read_to_string("input/day22.txt").unwrap();

    let lns: Vec<u64> = contents.lines().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", lns);
    let nums = 2000;
    let mut results: Vec<u64> = Vec::new();

    lns.iter().for_each(|ln| {
        let mut result = ln.clone();
        for _ in 0..nums {
            result = step(result);
        }
        results.push(result);
    });
    println!("{:?}", results);
    println!("{}", results.iter().sum::<u64>());
}

fn step(mut ln: u64) -> u64 {
    ln = mix(ln, ln * 64);
    ln = prune(ln);
    let m = ln as f64 / 32 as f64;
    ln = mix(ln, m.floor() as u64);
    ln = prune(ln);
    ln = mix(ln, ln * 2048);
    prune(ln)
}

fn mix(ln: u64, mix: u64) -> u64 {
    ln ^ mix
}

fn prune(ln: u64) -> u64 {
    ln % 16777216
}
