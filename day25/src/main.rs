use std::fs;

fn main() {
    let contents = fs::read_to_string("input/test.txt").unwrap();
    let contents = fs::read_to_string("input/day25.txt").unwrap();

    // println!("{}", contents);

    // contents.split("\n\n").for_each(|el| println!("a{}", el));
    let mut locks: Vec<Vec<String>> = Vec::new();
    let mut keys: Vec<Vec<String>> = Vec::new();
    let mut keynums: Vec<[i8; 5]> = Vec::new();
    let mut locknums: Vec<[i8; 5]> = Vec::new();
    contents.split("\n\n").for_each(|el| {
        let mut key = false;
        let mut obj: Vec<String> = Vec::new();
        el.split("\n").enumerate().for_each(|(i, ln)| {
            if i == 0 && ln == "....." {
                key = true;
            }
            obj.push(ln.to_string());
        });
        if key {
            keys.push(obj.clone());
            keynums.push(getnums(obj));
        } else {
            locks.push(obj.clone());
            locknums.push(getnums(obj));
        }
    });

    let mut combos = 0;
    keynums.iter().for_each(|key| {
        locknums.iter().for_each(|lock| {
            let mut combo = true;
            for i in 0..5 {
                if key[i] + lock[i] >= 6 {
                    combo = false;
                }
            }
            if combo {
                combos += 1;
            }
        })
    });

    println!("{}", combos);
}

fn getnums(obj: Vec<String>) -> [i8; 5] {
    let mut nums = [-1; 5];
    obj.iter().for_each(|ln| {
        ln.chars().enumerate().for_each(|(i, c)| {
            if c == '#' {
                nums[i] += 1;
            }
        })
    });
    nums
}
