use std::fs;

fn main() {
    // let file_path = "input/test2.txt";
    // let file_path = "input/test.txt";
    let file_path = "input/day9.txt";
    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{contents}");
    println!("len: {}", contents.len());

    // let mut map: Vec<char> = Vec::new();
    let mut map: String = String::new();
    let mut freespace = 0;
    let mut sortmap: String = String::new();
    let mut sortmap_rev: String = String::new();
    let mut i = 0;

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
                map = format!("{}{}", map, i / 2);
            }
        } else {
            for _x in 0..c as i8 - 48 {
                map = format!("{}{}", map, ".");
            }
        }
        i += 1;
        if i == 20 {
            i = 0;
        }
    });
    println!("{}, {}", map, map.len());
    sortmap_rev = map.clone();
    map.chars().for_each(|c| {
        if sortmap.len() + freespace < map.len() {
            if c.is_digit(10) {
                sortmap = format!("{}{}", sortmap, c);
            } else {
                sortmap_rev = format!("{}{}", sortmap_rev, c);
                while sortmap_rev.chars().last().unwrap() == '.' {
                    freespace += 1;
                    sortmap_rev.pop();
                }
                sortmap = format!("{}{}", sortmap, sortmap_rev.chars().last().unwrap());
                sortmap_rev.pop();
            }
        }
    });
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
    println!("{}, {}", sortmap, sortmap.len());
    let mut prevchar = 0;
    let mut mult: i128 = 0;
    let mut sum: i128 = 0;
    let mut count: i128 = 0;
    sortmap.chars().for_each(|c| {
        if c.is_digit(10) {
            // if prevchar == 9 && c.to_digit(10).unwrap() == 0 {
            //     mult += 10;
            // }
            sum += (c.to_digit(10).unwrap() as i128 + mult) * count;
            // println!("sum: {}, i: {}, c: {}", sum, count, c);
            count += 1;
            prevchar = c.to_digit(10).unwrap();
        }
    });
    // for (i, c) in sortmap.chars().enumerate() {
    //     if c.is_digit(10) {
    //         sum += (c.to_digit(10).unwrap() * i as u32) as i128;
    //         println!("sum: {}, i: {}, c: {}", sum, i, c);
    //     }
    // }
    println!("sum: {}", sum);
    println!("count: {}", count);
    println!("sortmap: {}", sortmap.len());
    println!("map len: {}", map.len());
    println!("sortmap+free len: {}", sortmap.len() + freespace);
    println!("orig len: {}", contents.len());
    //5536110224
    //89277257350 -- too small
}
