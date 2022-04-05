// Day 1, part 1
use std::fs;

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

    let mut fst: u32 = 1;
    let mut old: u32 = 0;
    let mut tot: u32 = 0;
    for w in s.split('\n') {
        if w.trim() == "" {
            continue;
        }
        let cur: u32 = w.trim().parse().expect("whoops");
        old = (fst * cur) + old;
        if cur > old {
            tot += 1;
        }
        old = cur;
        fst = 0;
    }
    println!("tot: {}", tot);
}
