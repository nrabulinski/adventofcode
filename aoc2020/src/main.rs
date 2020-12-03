use clap::{Arg, App, Error, ErrorKind};

mod day1;
mod day2;
mod day3;

macro_rules! days {
    ($d:expr, $p:expr => $($day:literal $name:ident),+) => {
        match ($d, $p) {
            $(
                ($day, 1) => $name::part1(),
                ($day, 2) => $name::part2(),
            )+
            _ => Error::with_description(&format!("Couldn't find day {} part {}", $d, $p), ErrorKind::InvalidValue).exit()
        }
    }
}

fn main() {
    let matches = App::new("Advent of code 2020")
        .author("Nikodem Rabuliński <nikodem@rabulinski.com>")
        .arg(Arg::with_name("day")
            .short("d")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("part")
            .short("p")
            .takes_value(true)
            .required(true))
        .get_matches();
    let day: u8 = matches.value_of("day").and_then(|d| d.parse().ok()).unwrap();
    let part: u8 = matches.value_of("part").and_then(|d| d.parse().ok()).unwrap();
    days! {
        day, part =>
        1 day1, 2 day2, 3 day3
    }
}
