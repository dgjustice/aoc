// Day 1, part 2
use std::fs;

fn sum(sl: &[u32]) -> u32 {
    let mut t: u32 = 0;
    for x in sl {
        println!("x: {}", x);
        t += x;
    }
    println!("");
    return t;
}

fn main() {
    // let s = "
    // 199
    // 200
    // 208
    // 210
    // 200
    // 207
    // 240
    // 269
    // 260
    // 263
    // ";

    let s = fs::read_to_string("input")
      .expect("Something went wrong reading the file");
    let mut vec = Vec::new();

    for w in s.split('\n') {
        if w.trim() == "" {
            continue;
        }
        let cur: u32 = w.trim().parse().expect("whoops");
        vec.push(cur);
    }

    let mut idx: usize = 0;
    let mut cur: u32 = sum(&vec[0..3]);
    let mut inc: u32 = 0;
    while idx < (vec.len()-2) {
        let t = sum(&vec[idx..idx+3]);
        if t > cur {
            inc += 1;
        }
        cur = t;
        idx += 1;
    }
    println!("tot: {}", inc);
}
