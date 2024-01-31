mod cli;
mod data_structures;
mod dynamic_programming;
mod generate_test_data;
mod logger;
mod math_and_numbers;
mod string_manipulation;
use crate::dynamic_programming::combinatorics::*;
use crate::logger::setup_logging;
use clap::{arg, command, Arg, ArgAction, ArgMatches, Command, Error};
use generate_test_data::generate_number_list;
use num_format::{Locale, ToFormattedString};
use tracing::{info, level_filters::LevelFilter, Level};
struct Solution;

use crate::cli::parse_args;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();

    let verbosity = matches.get_one::<u8>("verbose").unwrap_or(&0);
    setup_logging(*verbosity);

    if let Some(_) = matches.get_one::<bool>("generate-numbers") {
        handle_generate_numbers(&matches)?;
    }

    handle_combinatorics(&matches)?;

    Ok(())
}

fn handle_generate_numbers(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let num = *matches.get_one::<i64>("num").unwrap_or(&0);
    let output = matches.get_one::<String>("output");

    if num > 0 {
        let output_fp = generate_number_list(num, output.as_deref())?;
        println!("Numbers written to file: {}", output_fp);
    }

    Ok(())
}
fn handle_combinatorics(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Logic for "choose" and "permutation" commands
    if let Some(mut values) = matches.get_many::<u128>("choose") {
        let n = *values.next().unwrap();
        let k = *values.next().unwrap();

        let res = choose(n as u128, k as u128)?;
        if res >= 1000000 {
            eprintln!("{} permutation {} = {:e}", n, k, res);
        } else {
            eprintln!(
                "{} choose {} = {}",
                n,
                k,
                res.to_formatted_string(&Locale::en)
            );
        }
    }
    if let Some(mut values) = matches.get_many::<u128>("permutation") {
        let n = *values.next().unwrap();
        let k = *values.next().unwrap();

        let res = permutation(n as u128, k as u128)?;
        if res >= 1000000 {
            eprintln!("{} permutation {} = {:e}", n, k, res);
        } else {
            eprintln!(
                "{} permutation {} = {}",
                n,
                k,
                res.to_formatted_string(&Locale::en)
            );
        }
    }
    Ok(())
}
