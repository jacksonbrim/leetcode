mod add_two_nums;
mod combinatorics;
mod generate_test_data;
mod int_to_roman;
mod letter_combinations_phone_num;
mod longest_common_prefix;
mod longest_substring;
mod manachers_algo;
mod median_sorted_arrays;
mod most_water;
mod palindrome_number;
mod palindromic_substring;
mod regex_matching;
mod reverse_int;
mod roman_to_int;
mod string_to_int;
mod three_sum;
mod three_sum_closest;
mod two_sum;
mod zigzag;

use clap::{arg, command, Arg, ArgAction, ArgMatches, Command, Error};
use generate_test_data::generate_number_list;
use tracing::{info, level_filters::LevelFilter, Level};
use tracing_subscriber::FmtSubscriber;

struct Solution;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("2023 Advent of Code")
        .author("Jackson Brim")
        .version("0.1.0")
        .about("A program for running leetcode solutions")
        .arg(
            Arg::new("generate-numbers")
                .short('g')
                .long("generate-numbers")
                .action(clap::ArgAction::SetTrue)
                .help("Generate numbers and output to file (if specified with --output)"),
        )
        .arg(
            Arg::new("num")
                .short('n')
                .long("num")
                .value_parser(clap::value_parser!(i64))
                .help("The number of elements to generate"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .action(ArgAction::Set)
                .help("Specify the output file for generated numbers"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::Count)
                .help(
                    "Use the -v flag to set the verbosity level:
                OFF (default),
                -v => INFO,
                -vv => DEBUG,
                -vvv => TRACE",
                ),
        )
        .get_matches();

    let generate_numbers = matches.contains_id("generate-numbers");
    let num = *matches.get_one::<i64>("num").unwrap_or(&0);
    let output = matches.get_one::<String>("output");

    let verbosity = matches.get_one::<u8>("verbose").unwrap_or(&0);

    let tracing_level = match verbosity {
        0 => LevelFilter::OFF,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        3 | _ => LevelFilter::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing_level)
        .init();

    let tracing_level_str = format!("Tracing Level: {tracing_level}");
    println!("verbosity level: {:?}", tracing_level);

    // call programs
    if generate_numbers && num > 0 {
        if let Some(output_file) = output {
            // Call generate_number_list with the number of elements and the file path
            let output_fp = generate_test_data::generate_number_list(num, Some(output_file))?;
            println!("Numbers written to file: {}", output_fp);
        } else {
            let output_fp = generate_number_list(num, None)?;
            println!("Numbers written to file: {}", output_fp);
        }
    }

    Ok(())
}
