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

use crate::cli::cli::{
    parse_args,
    Combinatorics::{Choose, Permutation},
};
use crate::cli::cli_utils;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();

    let verbosity = matches.get_one::<u8>("verbose").unwrap_or(&0);
    setup_logging(*verbosity);

    match matches.subcommand() {
        Some(("generate-numbers", sub_m)) => {
            cli_utils::handle_generate_numbers(&sub_m)?;
        }
        Some(("choose", sub_m)) => {
            cli_utils::handle_combinatorics(Choose, &sub_m)?;
        }
        Some(("permutation", sub_m)) => {
            cli_utils::handle_combinatorics(Permutation, &sub_m)?;
        }
        Some(("add-two-numbers", sub_m)) => {
            cli_utils::handle_add_two_numbers(sub_m)?;
        }
        Some(("int-to-roman", sub_m)) => {
            cli_utils::handle_int_to_roman(sub_m)?;
        }
        Some(("find-median-of-sorted-arrays", sub_m)) => {
            cli_utils::handle_find_median_from_sorted_arrays(sub_m)?;
        }
        Some(("most-water", sub_m)) => {
            cli_utils::handle_most_water(sub_m)?;
        }
        Some(("valid-parenthesis", sub_m)) => {
            cli_utils::handle_valid_parenthesis(sub_m)?;
        }
        Some(("generate-completions", sub_m)) => {
            cli_utils::handle_shell_completions(sub_m)?;
        }

        _ => unreachable!("subcommand required"),
    }

    Ok(())
}
