static INPUT: &str = include_str!("../input/day1");

fn common() -> Vec<u32> {
    let mut vals: Vec<u32> = INPUT.split('\n').filter_map(|line| line.parse().ok()).collect();
    vals.sort_unstable();
    vals
}

pub fn part1() {
    let vals = common();
    'outer: for (i, &a) in vals.iter().enumerate() {
        for &b in (&vals[i + 1..]).iter().rev() {
            if a + b < 2020 {
                continue 'outer;
            }
            if a + b == 2020 {
                println!("{} * {} = {}", a, b, a * b);
                return;
            }
        }
    }
}

pub fn part2() {
    let vals = common();
    let len = vals.len();
    'outer: for (i, &a) in vals.iter().enumerate() {
        for (j, &b) in (&vals[i + 1..]).iter().rev().enumerate() {
            for &c in (&vals[i + 1..len - j - 1]).iter() {
                if a + b + c < 2020 {
                    continue 'outer;
                }
                if a + b + c == 2020 {
                    println!("{} * {} * {} = {}", a, b, c, a*b*c);
                }
            }
        }
    }
}