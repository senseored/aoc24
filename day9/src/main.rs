use std::{fs, usize};

fn main() {
    // let file_path = "input/test2.txt";
    // let file_path = "input/test.txt";
    let file_path = "input/day9.txt";
    // println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    // let contents = "12345";

    // println!("{contents}");
    let mut valuemap: String = String::new();
    let mut emptymap: String = String::new();
    let mut freevec = 0;
    let mut i = 0;
    let mut vecmap: Vec<i32> = Vec::new();
    let mut vecmap2: Vec<i32> = Vec::new();
    let mut sortvecmap: Vec<i32> = Vec::new();
    let mut maplen: usize = 0;
    let mut part3: Vec<i32> = Vec::new();

    contents.chars().for_each(|c| {
        if i % 2 == 0 {
            for _x in 0..c as i8 - 48 {
                vecmap.push(i / 2);
                vecmap2.push(i / 2);
            }
        } else {
            for _x in 0..c as i8 - 48 {
                vecmap.push(-1);
                freevec += 1;
            }
        }
        if c.is_digit(10) {
            maplen += c.to_digit(10).unwrap() as usize;
        }
        i += 1;
    });
    let mut vecmap3 = vecmap2.clone();
    for (x, c) in contents.chars().enumerate() {
        if x % 2 == 0 {
            valuemap = format!("{}{}", valuemap, c);
        } else {
            emptymap = format!("{}{}", emptymap, c);
        }
    }
    vecmap.iter().for_each(|n| {
        if sortvecmap.len() + freevec < vecmap.len() {
            if !n.is_negative() {
                sortvecmap.push(*n);
            } else {
                sortvecmap.push(*vecmap2.last().unwrap());
                vecmap2.pop();
            }
        }
    });
    let mut sum: i128 = 0;
    let mut count2 = 0;
    let mut count: i128 = 0;
    sortvecmap.iter().for_each(|n| {
        if count2 < maplen {
            sum += *n as i128 * count;
            count2 += 1;
            count += 1;
        }
    });

    println!("sum: {}", sum);
    println!("count: {}", count);
    println!("sortmap: {}", sortvecmap.len());
    println!("map len: {}", maplen);
    println!("vecmap len: {}", vecmap.len());
    println!("sortmap+free len: {}", sortvecmap.len() + freevec);
    println!("orig len: {}", contents.len());

    // println!("\nvaluemap: {}", valuemap);
    // println!("emptymap: {}", emptymap);
    // println!("contents: {}", contents);
    // println!("vecmap: {:?}", vecmap);

    let mut movecount = 0;
    let mut something = true;
    let mut something2 = true;
    // while something {
    //     let mut addlist: Vec<usize> = Vec::new();
    //     let mut remlist: Vec<usize> = Vec::new();
    //     for (i, n) in vecmap.iter().enumerate() {
    //         if n.is_negative() {
    //             addlist.push(i);
    //             freecount += 1;
    //         } else {
    //             let mut lastvalue = vecmap.last().unwrap().clone();
    //             let mut seqcount = 0;
    //             let mut vecx = vecmap.len();
    //             if freecount > 0 {
    //                 for (x, c) in vecmap.iter().rev().enumerate() {
    //                     // println!(
    //                     //     "c: {}, lastvalue: {}, seqcount: {}, freecount: {}",
    //                     //     c, lastvalue, seqcount, freecount
    //                     // );
    //                     if *c == lastvalue && c.is_positive() {
    //                         seqcount += 1;
    //                     } else {
    //                         if seqcount <= freecount && lastvalue > 0 {
    //                             for _z in 0..seqcount {
    //                                 freecount -= 1;
    //                                 if remlist.is_empty() {
    //                                     remlist.push(vecx - x + seqcount);
    //                                     seqcount -= 1;
    //                                 } else if remlist.last().unwrap() > &(vecx - x + seqcount) {
    //                                     remlist.push(vecx - x + seqcount);
    //                                     seqcount -= 1;
    //                                 }
    //                             }
    //                         }
    //                         seqcount = 1;
    //                     }
    //                     lastvalue = *c;
    //                 }
    //             }
    //         }
    //     }
    //     println!("vecmap: {:?}", vecmap);
    //     println!("addlist: {:?}, remlist: {:?}", addlist, remlist);
    //     for i in 0..remlist.len() {
    //         vecmap[addlist[i]] = vecmap[remlist[i] - 1] as i32;
    //     }
    //     for n in remlist.iter() {
    //         vecmap.remove(*n - 1);
    //     }
    //     if remlist.is_empty() {
    //         something = false;
    //     }
    // }
    let mut checked = 0;
    let mut lastvalue = -1;
    // let mut lastvalue = vecmap.last().unwrap().clone();
    while something {
        let mut lastvalue = -1;
        let mut addlist: Vec<usize> = Vec::new();
        let mut remlist: Vec<usize> = Vec::new();
        let mut movecount = 0;
        let mut freevec = 0;
        let mut letsgo = true;
        for i in checked..vecmap.len() {
            // addlist = Vec::new();
            // remlist = Vec::new();
            if letsgo {
                let c = vecmap[vecmap.len() - 1 - i];
                // if c != lastvalue {
                //     movecount = 0;
                // }
                if lastvalue > 0 && c == lastvalue {
                    movecount += 1;
                } else {
                    if lastvalue.is_positive() {
                        movecount += 1;
                        let mut leggo = true;
                        for y in 0..vecmap.len() {
                            // if y <= vecmap.len() - 1 - i {
                            if y <= vecmap.len() - checked {
                                if leggo {
                                    if vecmap[y].is_negative() {
                                        freevec += 1;
                                    } else {
                                        if movecount > freevec {
                                            freevec = 0;
                                        }
                                        //     println!(
                                        //     "movecount: {}, freevec: {} lastvalue: {}, index: {}, i: {}, y: {}",
                                        //     movecount,
                                        //     freevec,
                                        //     lastvalue,
                                        //     vecmap.len() - 1 - i, i, y
                                        // );
                                        if freevec >= movecount {
                                            for z in 0..movecount {
                                                addlist.push(y - freevec + z);
                                                remlist.push(vecmap.len() - 1 - i + movecount - z);
                                                lastvalue = -1;
                                            }
                                            // freevec -= movecount;
                                            movecount = 0;
                                            freevec = 0;
                                            leggo = false;
                                            // } else {
                                            //     freevec = 0;
                                        }
                                    }
                                }
                            }
                        }
                        freevec = 0;
                        movecount = 0;
                        letsgo = false;
                    }
                }
                lastvalue = c;
                checked = i;
            }
        }
        // println!("checked: {}", checked);
        // println!("addlist: {:?}, remlist: {:?}", addlist, remlist);
        // println!("movecount: {}, lastvalue: {}", movecount, lastvalue);
        for i in 0..addlist.len() {
            vecmap[addlist[i]] = vecmap[remlist[i]] as i32;
            vecmap[remlist[i]] = -1;
        }
        // println!("vecmap: {:?}", vecmap);
        if checked == vecmap.len() - 1 {
            something = false;
        }
    }
    // for i in 0..remlist.len() {
    //     vecmap[addlist[i]] = vecmap[remlist[i] - 1] as i32;
    // }
    // for n in remlist.iter() {
    //     vecmap.remove(*n - 1);
    // }
    // if remlist.is_empty() {
    //     something = false;
    // }
    // }
    // vecmap.iter().for_each(|n| {
    //     if part3.len() + freevec < vecmap.len() {
    //         if n.is_negative() {
    //             freecount += 1;
    //         } else {
    //             if freecount > 0 {
    //                 println!("freecount: {}", freecount);
    //                 let mut seqcount = 0;
    //                 let mut x = 0;
    //                 let mut lastvalue = vecmap3.last().unwrap().clone();
    //                 let mut remlist: Vec<usize> = Vec::new();
    //                 for c in vecmap3.iter().rev() {
    //                     println!("c: {}, lastvalue: {}, seqcount: {}", c, lastvalue, seqcount);
    //                     if *c == lastvalue && c.is_positive() {
    //                         seqcount += 1;
    //                     } else {
    //                         if seqcount <= freecount {
    //                             for _z in 0..seqcount {
    //                                 part3.push(lastvalue);
    //                                 freecount -= 1;
    //                                 remlist.push(vecmap3.len() - x - 1);
    //                             }
    //                             println!("freecount: {}", freecount);
    //                         }
    //                         seqcount = 1;
    //                     }
    //                     lastvalue = *c;
    //                     x += 1;
    //                 }
    //                 for n in remlist.iter().rev() {
    //                     vecmap3.remove(*n);
    //                 }
    //             }
    //             freecount = 0;
    //             part3.push(*n);
    //         }
    //     }
    // });
    println!("vecmap: {:?}", vecmap);
    let mut sum: i128 = 0;
    let mut count2 = 0;
    let mut count: i128 = 0;
    vecmap.iter().for_each(|n| {
        if n.is_positive() {
            sum += *n as i128 * count;
        }
        count += 1;
    });

    println!("sum: {}", sum);
    println!("count: {}", count);
    //6304576012713
}
