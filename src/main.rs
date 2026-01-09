mod day_1;
mod day_2;
mod day_3;

mod inputs;

use ferris_says::say;
use std::env;
use std::io::{BufWriter, stdout};
use std::process;

fn main() {
    let day = parse_args().unwrap_or_else(|err| {
        eprintln!("Error in arguments: {err}");
        process::exit(1);
    });
    let message = run_day(day);

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say(&message, 10, &mut writer).unwrap();
}

fn run_day(day: u32) -> String {
    match day {
        1 => day_1::run(),
        2 => day_2::run(),
        3 => day_3::run(),
        _ => unimplemented!(),
    }
}

fn parse_args() -> Result<u32, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: cargo run <day>");
    }

    match args[1].trim().parse() {
        Ok(day) if (1..24).contains(&day) => Ok(day),
        _ => Err("Provide a valid day in [1-24]"),
    }
}
