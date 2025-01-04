use std::fs;

fn main() {
    let contents = fs::read_to_string("input/test.txt").unwrap();
    // let contents = fs::read_to_string("input/day22.txt").unwrap();

    let lns: Vec<u64> = contents.lines().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", lns);
    let nums = 2000;
    let mut results: Vec<u64> = Vec::new();
    let mut reslist: Vec<Vec<u64>> = Vec::new();

    lns.iter().for_each(|ln| {
        let mut result = ln.clone();
        let mut res = Vec::new();
        for _ in 0..nums {
            result = step(result);
            res.push(result);
        }
        results.push(result);
        reslist.push(res);
    });
    println!("{:?}", results);

    // println!("{}", results.iter().sum::<u64>());
    let mut changes: Vec<Vec<(i8, i8)>> = Vec::new();
    reslist
        .iter()
        .for_each(|x| changes.push(ln_chn(x.to_vec())));
    println!("{:?}", changes);
    // reslist
    //     .iter()
    //     .for_each(|x| println!("{:?}", ln_chn(x.to_vec())));
    // println!("{:?}", ln_chn(reslist));
}

fn step(mut ln: u64) -> u64 {
    ln = mix(ln, ln * 64);
    ln = prune(ln);
    let m = ln as f64 / 32_f64;
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

fn ln_chn(nums: Vec<u64>) -> Vec<(i8, i8)> {
    let mut chn: Vec<(i8, i8)> = Vec::new();
    nums.iter().enumerate().for_each(|(i, n)| {
        chn.push((
            n.to_string().chars().last().unwrap().to_digit(10).unwrap() as i8,
            0,
        ));
        if i > 0 {
            chn[i].1 = chn[i].0 - chn[i - 1].0;
        }
        // println!("{}, {:?}", n, chn[i]);
    });
    chn
}
