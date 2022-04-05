// Day 2, part 1
use std::fs;

fn update_pos(pos: (u32, u32), cmd: &str, mag: u32) -> (u32, u32) {
    match cmd {
        "forward" => (pos.0 + mag, pos.1),
        "up" => (pos.0, pos.1 - mag),
        "down" => (pos.0, pos.1 + mag),
        _ => (pos.0, pos.1),
    }
}

fn main() {
    // let s = "
    // forward 5
    // down 5
    // forward 8
    // up 3
    // down 8
    // forward 2
    // ";

    let s = fs::read_to_string("input")
      .expect("Something went wrong reading the file");

    let mut pos: (u32, u32) = (0, 0);
    for w in s.split('\n') {
        if w.trim() == "" {
            continue;
        }
        let parts: Vec<&str> = w.trim().split(' ').collect();
        let mag: u32 = parts[1].trim().parse().expect("whoops");
        pos = update_pos(pos, parts[0], mag);
    }

    println!("pos: {}, {}", pos.0, pos.1);
    // >>> 1957*955
    // 1868935
}
