use std::fs;

fn main() {
    // let file_path = "input/test2.txt";
    // let file_path = "input/test.txt";
    let file_path = "input/day9.txt";
    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let contents = "12345";

    // println!("{contents}");
    // println!("len: {}", contents.len());

    // let mut map: Vec<char> = Vec::new();
    let mut map: String = String::new();
    let mut freevec = 0;
    let mut freespace = 0;
    let mut sortmap: String = String::new();
    let mut sortmap_rev: String = String::new();
    let mut i = 0;
    let mut vecmap: Vec<i32> = Vec::new();
    let mut vecmap2: Vec<i32> = Vec::new();
    let mut sortvecmap: Vec<i32> = Vec::new();
    let mut sortvecmap2: Vec<i32> = Vec::new();
    let mut revvecmap: Vec<i32> = Vec::new();
    let mut maplen: usize = 0;

    // contents.chars().for_each(|c| {
    contents.chars().for_each(|c| {
        // map.push(c);
        // println!("c: {}, i: {}", c as i8, i);
        // println!(
        //     "is numeric: {} - number: {}",
        //     c.is_numeric(),
        //     c.to_digit(10).unwrap()
        // );
        if i % 2 == 0 {
            for _x in 0..c as i8 - 48 {
                // map = format!("{}{}", map, i / 2);
                vecmap.push(i / 2);
                vecmap2.push(i / 2);
            }
        } else {
            for _x in 0..c as i8 - 48 {
                // map = format!("{}{}", map, ".");
                vecmap.push(-1);
                freevec += 1;
            }
        }
        i += 1;
        if c.is_digit(10) {
            maplen += c.to_digit(10).unwrap() as usize;
        }
        // if i == 20 {
        //     i = 0;
        // }
    });
    // println!("{}, {}", map, map.len());
    // println!("{:?}", vecmap);
    sortmap_rev = map.clone();
    // while sortmap.len() + freespace < map.len() {
    //     map.chars().for_each(|c| {
    //         if sortmap.len() + freespace < map.len() {
    //             if c.is_digit(10) {
    //                 sortmap = format!("{}{}", sortmap, c);
    //             } else {
    //                 sortmap_rev = format!("{}{}", sortmap_rev, c);
    //                 while sortmap_rev.chars().last().unwrap() == '.' {
    //                     freespace += 1;
    //                     sortmap_rev.pop();
    //                 }
    //                 sortmap = format!("{}{}", sortmap, sortmap_rev.chars().last().unwrap());
    //                 sortmap_rev.pop();
    //             }
    //         }
    //     });
    // }
    revvecmap = vecmap.clone();
    println!("{:?}", vecmap);
    vecmap.iter().for_each(|n| {
        if sortvecmap2.len() + freevec < vecmap.len() {
            if !n.is_negative() {
                sortvecmap2.push(*n);
            } else {
                sortvecmap2.push(*vecmap2.last().unwrap());
                vecmap2.pop();
            }
        }
        // println!(
        //     "sortvevclen: {}, freevec: {}, vecmaplen, {}",
        //     sortvecmap.len(),
        //     freevec,
        //     vecmap.len()
        // );
        // if sortvecmap.len() + freevec < vecmap.len() {
        //     if !n.is_negative() {
        //         sortvecmap.push(*n);
        //     } else {
        //         revvecmap.push(*n);
        //         while revvecmap.last().unwrap().is_negative() {
        //             freevec += 1;
        //             revvecmap.pop();
        //         }
        //         sortvecmap.push(*revvecmap.last().unwrap());
        //         revvecmap.pop();
        //     }
        // println!("n: {}", n);
        // println!(
        //     "sortvevclen: {}, freevec: {}, vecmaplen, {}",
        //     sortvecmap.len(),
        //     freevec,
        //     vecmap.len()
        // );
        // if *n == -1 {
        //     // freevec += 1;
        //     // while revvecmap[revvecmap.len() - 1] != -1 {
        //     while *revvecmap.last().unwrap() == -1 {
        //         freevec += 1;
        //         revvecmap.pop();
        //     }
        //     sortvecmap.push(*revvecmap.last().unwrap());
        //     revvecmap.pop();
        // } else {
        //     sortvecmap.push(*n);
        // }
        // }
    });
    sortvecmap2.iter().for_each(|n| {
        if n.is_negative() {
            println!("yo i done messed up");
        }
    });
    println!("{:?}", sortvecmap2);
    // revvecmap.pop();
    // sortvecmap.push(*revvecmap.last().unwrap());
    // revvecmap.pop();
    // println!("{}", revvecmap.last().unwrap());
    // println!("{:?}, {}", sortvecmap, sortvecmap.len());
    // map.chars().rev().for_each(|c| {
    //     if c != '.' {
    //         sortmap_rev = format!("{}{}", sortmap_rev, c);
    //     }
    // });
    // map.chars().for_each(|c| {
    //     if c == '.' {
    //         freespace += 1;
    //     } else {
    //         sortmap = format!("{}{}", sortmap, c);
    //     }
    // });
    // for _i in 0..freespace {
    //     sortmap = format!("{}{}", sortmap, '.');
    // }
    // println!("sortmap: {}", sortmap_rev);
    // println!("{}, {}", sortmap, sortmap.len());
    let mut prevchar = 0;
    let mut mult: i128 = 0;
    let mut sum: i128 = 0;
    let mut count2 = 0;
    let mut count: i128 = 0;
    sortvecmap2.iter().for_each(|n| {
        // println!("n: {}, sum: {}, count: {}", n, sum, count);
        if count2 < maplen {
            sum += *n as i128 * count;
            count2 += 1;
            count += 1;
        }
    });
    // sortmap.chars().for_each(|c| {
    //     if c.is_digit(10) {
    //         // if prevchar == 9 && c.to_digit(10).unwrap() == 0 {
    //         //     mult += 10;
    //         // }
    //         sum += (c.to_digit(10).unwrap() as i128 + mult) * count;
    //         // println!("sum: {}, i: {}, c: {}", sum, count, c);
    //         count += 1;
    //         prevchar = c.to_digit(10).unwrap();
    //     }
    // });
    // for (i, c) in sortmap.chars().enumerate() {
    //     if c.is_digit(10) {
    //         sum += (c.to_digit(10).unwrap() * i as u32) as i128;
    //         println!("sum: {}, i: {}, c: {}", sum, i, c);
    //     }
    // }
    println!("sum: {}", sum);
    println!("count: {}", count);
    println!("sortmap: {}", sortvecmap.len());
    println!("map len: {}", maplen);
    println!("vecmap len: {}", vecmap.len());
    println!("sortmap+free len: {}", sortvecmap.len() + freevec);
    println!("orig len: {}", contents.len());
    //5536110224
    //89277257350 -- too small
    //6283665299144
    //24071685531123
    //24071685531123
    //6283404590840
}
