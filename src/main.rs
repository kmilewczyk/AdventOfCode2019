extern crate clap;
#[macro_use] extern crate log;
extern crate pretty_env_logger;
#[macro_use] extern crate anyhow;
#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;

use clap::{App, Arg, SubCommand};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

lazy_static! {
    static ref SUBCOMMANDS: Vec<&'static str> = vec!["day1_1", "day1_2", "day2_1", "day2_2", "day3_1",
    "day3_2", "day4_1", "day4_2", "day5", "day6_1"];
}

fn main() {
    pretty_env_logger::init();

    let matches =  {
        let mut app = App::new("Advent of Code 2019")
        .version("0.2")
        .author("Karol Milewczyk")
        .arg(Arg::with_name("input")
            .help("Path to input file")
            .short("f")
            .long("input")
            .takes_value(true));

        for subcommand in SUBCOMMANDS.iter() {
            app = app.subcommand(SubCommand::with_name(subcommand));
        }

        app.get_matches()
    };

    let filepath = matches.value_of("input").unwrap_or("input.txt");
    info!("Using file \"{}\" as input.", filepath);

    let command_result = match matches.subcommand() {
        ("day1_1", _) => { day_1::calculate_fuel(filepath.to_string()) },
        ("day1_2", _) => { day_1::calculate_fuel_extended(filepath.to_string()) },
        ("day2_1", _) => { day_2::program_alarm(filepath.to_string()) },
        ("day2_2", _) => { day_2::search_answer(filepath.to_string()) },
        ("day3_1", _) => { day_3::find_closest_crossing(filepath.to_string()) },
        ("day3_2", _) => { day_3::find_lowest_latency(filepath.to_string()) },
        ("day4_1", _) => { day_4::count_diffrent_passwords(filepath.to_string()) },
        ("day4_2", _) => { day_4::count_diffrent_passwords_part2(filepath.to_string()) },
        ("day5", _)   => { day_5::diagnostic_tests(filepath.to_string()) },
        ("day6_1", _) => { day_6::total_orbit_count(filepath.to_string()) },
        _ => { Err(anyhow!("Challenge is unspecified")) },
    };

    if let Err(error) = command_result {
        error!("{}", error);
    }
}
