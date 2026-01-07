use std::error::Error;
use std::{env, fs, process::exit};

mod day01;

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

        if !(1..=24).contains(&day) || !(1..=2).contains(&part) {
            return Err("Expected day to be between 1-24 and part between 1-2.");
        }

        Ok(Config {
            day,
            part,
            data_source,
        })
    }
}

fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let path = format!("{}/day{:02}.txt", config.data_source, config.day);
    let contents = fs::read_to_string(&path)?;

    let result = match (config.day, config.part) {
        (1, 1) => day01::part01(&contents),
        (1, 2) => day01::part02(&contents),
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
