extern crate clap;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
#[macro_use] extern crate anyhow;
#[macro_use] extern crate itertools;

use clap::{App, Arg, SubCommand};

mod day_1;
mod day_2;

fn main() {
    pretty_env_logger::init();

    let matches = App::new("Advent of Code 2019")
        .version("0.2")
        .author("Karol Milewczyk")
        .subcommand(SubCommand::with_name("day1_1"))
        .subcommand(SubCommand::with_name("day1_2"))
        .subcommand(SubCommand::with_name("day2_1"))
        .subcommand(SubCommand::with_name("day2_2"))
        .arg(Arg::with_name("input")
            .help("Path to input file")
            .short("f")
            .long("input")
            .takes_value(true))
        .get_matches();

    let filepath = matches.value_of("input").unwrap_or("input.txt");
    info!("Using file \"{}\" as input.", filepath);

    let command_result = match matches.subcommand() {
        ("day1_1", _) => { day_1::calculate_fuel(filepath.to_string()) },
        ("day1_2", _) => { day_1::calculate_fuel_extended(filepath.to_string()) },
        ("day2_1", _) => { day_2::program_alarm(filepath.to_string()) },
        ("day2_2", _) => { day_2::search_answer(filepath.to_string()) },
        _ => { Err(anyhow!("Challenge is unspecified")) },
    };

    if let Err(error) = command_result {
        error!("{}", error);
    }
}
