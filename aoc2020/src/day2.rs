static INPUT: &str = include_str!("../input/day2");
use regex::Regex;

#[derive(Debug)]
struct Pass<'a> {
    min: usize,
    max: usize,
    ch: &'a str,
    pass: &'a str,
}

pub fn part1() {
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
    let count = re.captures_iter(INPUT)
        .filter(|capture| {
            let min = capture.name("min").map(|min| min.as_str()).unwrap();
            let max = capture.name("max").map(|max| max.as_str()).unwrap();
            let ch = capture.name("letter").map(|letter| letter.as_str()).unwrap();
            // ^([^c]*c[^c]*){min,max}$ is the regex for checking validity of the password
            // It's slower than my other solutions but it's way cooler so I'll leave this
            Regex::new(&format!("^([^{ch}]*{ch}[^{ch}]*){{{min},{max}}}$", ch=ch, min=min, max=max))
                .unwrap().is_match(capture.name("password").map(|pass| pass.as_str()).unwrap())
        }).count();
    println!("{} valid passwords", count);
}

pub fn part2() {
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>\w): (?P<password>\w+)").unwrap();
    let count = re.captures_iter(INPUT)
        .filter(|capture| {
            let a: usize = capture.name("min").and_then(|a| a.as_str().parse().ok()).unwrap();
            let b: usize = capture.name("max").and_then(|b| b.as_str().parse().ok()).unwrap();
            let ch = capture.name("letter").map(|letter| letter.as_str().as_bytes()[0]).unwrap();
            let c = capture.name("password").map(|pass| pass.as_str().as_bytes()).unwrap();
            (c[a - 1] == ch) ^ (c[b - 1] == ch)
        }).count();
    println!("{} valid passwords", count);
}