static INPUT: &str = include_str!("../input/day3");

pub fn part1() {
    let res = INPUT.split('\n').enumerate().fold(0, |acc, (y, line)| match line.as_bytes()[(y * 3) % line.len()] {
        b'#' => acc + 1,
        _ => acc
    });
    println!("Encountered {} trees", res);
}

pub fn part2() {
    let res = INPUT.split('\n').enumerate().fold((0, 0, 0, 0, 0), |mut acc, (y, line)| {
        // part A
        if line.as_bytes()[y % line.len()] == b'#' {
            acc.0 += 1;
        }

        // part B
        if line.as_bytes()[(y * 3) % line.len()] == b'#' {
            acc.1 += 1;
        }

        // part C
        if line.as_bytes()[(y * 5) % line.len()] == b'#' {
            acc.2 += 1;
        }

        // part D
        if line.as_bytes()[(y * 7) % line.len()] == b'#' {
            acc.3 += 1;
        }

        // part E
        if y % 2 == 0 && line.as_bytes()[(y / 2) % line.len()] == b'#' {
            acc.4 += 1;
        }
        acc
    });
    let (a, b, c, d, e) = res;
    println!("{}", a * b * c * d * e);
}