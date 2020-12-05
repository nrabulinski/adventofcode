static INPUT: &str = include_str!("../input/day5");

fn parse_seat(s: &str) -> (u8, u8) {
    let mut min = 0;
    let mut max = 127;
    for &c in &s.as_bytes()[..7] {
        match c {
            b'F' => max -= (max - min + 1) / 2,
            b'B' => min += (max - min + 1) / 2,
            _ => unreachable!()
        }
    }
    assert_eq!(min, max);
    let row = min;
    let mut min = 0;
    let mut max = 7;
    for &c in &s.as_bytes()[7..] {
        match c {
            b'L' => max -= (max - min + 1) / 2,
            b'R' => min += (max - min + 1) / 2,
            _ => unreachable!()
        }
    }
    assert_eq!(min, max);
    let col = min;
    (row, col)
}

pub fn part1() {
    let m = INPUT.split('\n').map(parse_seat).map(|(row, col)| row as u32 * 8 + col as u32).max().unwrap();
    println!("{}", m);
}

pub fn part2() {
    let mut ids: Vec<_> = INPUT.split('\n').map(parse_seat).map(|(row, col)| row as u32 * 8 + col as u32).collect();
    ids.sort_unstable();
    for i in 1..ids.len() {
        let a = ids[i - 1];
        let b = ids[i];
        if b - a != 1 {
            println!("{}", b - 1);
            return;
        }
    }
}