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

use crate::cli::cli::parse_args;
use crate::cli::cli_utils;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();

    let verbosity = matches.get_one::<u8>("verbose").unwrap_or(&0);
    setup_logging(*verbosity);

    if let Some(_) = matches.get_one::<bool>("generate-numbers") {
        cli_utils::handle_generate_numbers(&matches)?;
    }

    cli_utils::handle_combinatorics(&matches)?;
    cli_utils::handle_add_two_numbers(&matches)?;

    Ok(())
}
