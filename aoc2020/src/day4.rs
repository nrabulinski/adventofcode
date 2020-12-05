use std::collections::HashMap;
use regex::Regex;
static INPUT: &str = include_str!("../input/day4");

fn common() -> impl Iterator<Item=HashMap<&'static str, &'static str>> {
    INPUT.split("\n\n").map(|pass| 
        pass
            .split_ascii_whitespace()
            .filter_map(|a| {
                let mut a = a.split(':');
                Some((a.next()?, a.next()?))
            })
            .collect()
    )
}

pub fn part1() {
    const REQUIRED: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let count = common().filter(|pass| {
        for field in REQUIRED {
            if !pass.contains_key(field) {
                return false
            }
        }
        true
    }).count();
    println!("{}", count);
}

macro_rules! unwrap {
    ($o:expr) => {
        match $o {
            Some(val) => val,
            _ => return false
        }
    };
}

enum Height {
    Cm(u32),
    In(u32)
}

impl Height {
    fn from_str(s: &str) -> Option<Self> {
        let re = Regex::new(r"(?P<len>\d+)(?P<unit>in|cm)").unwrap();
        let m = re.captures(s)?;
        let len = m.name("len").unwrap().as_str().parse().unwrap();
        match m.name("unit").unwrap().as_str() {
            "in" => Some(Height::In(len)),
            "cm" => Some(Height::Cm(len)),
            _ => None
        }
    }
}

pub fn part2() {
    let count = common().filter(|pass| {
        fn check_year(s: &str, min: u16, max: u16) -> bool {
            let re = Regex::new(r"^\d{4}$").unwrap();
            match re.find(s) {
                Some(m) => {
                    let val: u16 = m.as_str().parse().unwrap();
                    val >= min && val <= max
                },
                None => false
            }
        }
        check_year(unwrap!(pass.get("byr")), 1920, 2002) &&
        check_year(unwrap!(pass.get("iyr")), 2010, 2020) &&
        check_year(unwrap!(pass.get("eyr")), 2020, 2030) &&
        (match unwrap!(pass.get("hgt").and_then(|&hgt| Height::from_str(hgt))) {
            Height::In(hgt) => hgt >= 59 && hgt <= 76,
            Height::Cm(hgt) => hgt >= 150 && hgt <= 193
        }) &&
        Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(unwrap!(pass.get("hcl"))) &&
        Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(unwrap!(pass.get("ecl"))) &&
        Regex::new(r"^\d{9}$").unwrap().is_match(unwrap!(pass.get("pid")))
    }).count();
    println!("{}", count);
}