use std::error::Error;
use std::{env, fs, process::exit};

use aoc2025_rust::{
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12,
};

struct Config {
    day: u32,
    part: u32,
    data_source: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("Expected day, part and data source arguments.");
        }

        let day: u32 = args[1]
            .parse()
            .expect("First argument should be a day number.");
        let part: u32 = args[2]
            .parse()
            .expect("Second argument should be part number");
        let data_source = args[3].clone();

        if data_source != "data" && data_source != "example" {
            return Err("Third argument must be 'data' or 'example'.");
        }

        if !(1..=12).contains(&day) || !(1..=2).contains(&part) {
            return Err("Expected day to be between 1-12 and part between 1-2.");
        }

        Ok(Config {
            day,
            part,
            data_source,
        })
    }
}

fn run(config: Config) -> Result<u64, Box<dyn Error>> {
    let path = format!("{}/day{:02}.txt", config.data_source, config.day);
    let contents = fs::read_to_string(&path)?;

    let result = match (config.day, config.part) {
        (1, 1) => day01::part01(&contents),
        (1, 2) => day01::part02(&contents),
        (2, 1) => day02::part01(&contents),
        (2, 2) => day02::part02(&contents),
        (3, 1) => day03::part01(&contents),
        (3, 2) => day03::part02(&contents),
        (4, 1) => day04::part01(&contents),
        (4, 2) => day04::part02(&contents),
        (5, 1) => day05::part01(&contents),
        (5, 2) => day05::part02(&contents),
        (6, 1) => day06::part01(&contents),
        (6, 2) => day06::part02(&contents),
        (7, 1) => day07::part01(&contents),
        (7, 2) => day07::part02(&contents),
        (8, 1) => day08::part01(&contents),
        (8, 2) => day08::part02(&contents),
        (9, 1) => day09::part01(&contents),
        (9, 2) => day09::part02(&contents),
        (10, 1) => day10::part01(&contents),
        (10, 2) => day10::part02(&contents),
        (11, 1) => day11::part01(&contents),
        (11, 2) => day11::part02(&contents),
        (12, 1) => day12::part01(&contents),
        (12, 2) => day12::part02(&contents),
        _ => panic!(
            "Specified day {} part {} not implemented.",
            config.day, config.part
        ),
    };

    Ok(result)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {err}");
        exit(1)
    });

    let result = run(config).unwrap_or_else(|err| {
        eprintln!("Error while calculating result: {err}");
        exit(1)
    });

    println!("{result}");
}
