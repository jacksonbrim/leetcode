use clap::{command, Arg, ArgAction, ArgMatches, Command};
pub enum Combinatorics {
    Choose,
    Permutation,
}

pub fn parse_args() -> ArgMatches {
    Command::new("Leetcode Solutions")
        .author("Jackson Brim")
        .version("0.1.0")
        .about("A program for running leetcode solutions")
        .subcommands([
            Command::new("generate-numbers")
                .about("Generate numbers and output to file")
                .arg(
                    Arg::new("nums")
                        .index(1)
                        .num_args(1)
                        .value_parser(clap::value_parser!(i64))
                        .help("The number of elements to generate")
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .action(ArgAction::Set)
                        .help("Specify the output file for generated numbers")
                ),
            Command::new("choose")
                .about("Calculate combinations")
                .long_about("Calculates combinations, requires two numbers where n > k and both > 0. In a combinatorics, n choose k represents a combination of n things taken k at a time without repetition")
                .arg(
                    Arg::new("n")
                    .index(1)
                        .required(true)
                        .value_parser(clap::value_parser!(u128)),
                )
                .arg(
                    Arg::new("k")
                    .index(2)
                        .required(true)
                        .value_parser(clap::value_parser!(u128)),
                ),
            Command::new("permutation")
                .about("Calculate combinations")
                .long_about("Calculates k-permutations of n. requires two numbers where n > k and both > 0. In a combinatorics, n permutation k represents all the different ways that a list can be arranged.")
                .arg(
                    Arg::new("n")
                    .index(1)
                        .required(true)
                        .value_parser(clap::value_parser!(u128))
                        .help("k permutation n: n is the total number of elements")
                )
                .arg(
                    Arg::new("k")
                    .index(2)
                        .required(true)
                        .value_parser(clap::value_parser!(u128))
                        .help("k permutation n: k is the subset of the total number of elements")
                )
                .arg(
                    Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(clap::ArgAction::Count)
                        .help("Set the verbosit level. OFF (default), -v => INFO, -vv => DEBUG, -vvv => TRACE"),
                ),
            Command::new("add-two-numbers")
                .about("Given two non-empty linked lists representing two non-negative integers stored in reverse order. Each of their nodes contians a single digit. Add the two numbers return the sum as a linked list.")
                .long_about("The digit of each number is converted into a linked list. The digits of each number are added, and the resulting linked list is returned as a number.")
                .arg(
                    Arg::new("num1")
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                )
                .arg(
                    Arg::new("num2")
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                ),
            Command::new("int-to-roman")
                .about("Converts an integer into a Roman Numeral. (e.g. 44 -> XLIV")
                .arg(
                    Arg::new("num")
                    .index(1)
                    .num_args(1)
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                    .help("Enter an integer from 1 to 3999.")
                ),
            Command::new("find-median-of-sorted-arrays")
                .about("Find the median from two sorted arrays. Arrays must comma separated and surrounded in double quotes. (e.g. \"1,2,3,4,5\" \"6,7,8,9\"Find the median from two sorted arrays. Arrays must comma separated and surrounded in double quotes. (e.g. \"1,2,3,4,5\" \"6,7,8,9\"")
                .arg(
                    Arg::new("arrays")
                    .index(1)
                    .num_args(2)
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                    .help("Find the median from two sorted arrays. Arrays must comma separated and surrounded in double quotes. (e.g. \"1,2,3,4,5\" \"6,7,8,9\""),
                ),
            Command::new("max-area")
                .about("Find the maximum area between any two heights from a list of heights. List must comma separated and surrounded in double quotes. (e.g. \"1,2,3,4,5\" \"6,7,8,9\"")
                .arg(
                    Arg::new("height-array")
                        .index(1)
                        .num_args(1)
                        .value_parser(clap::value_parser!(String))
                        .help("Array must comma separated and surrounded in double quotes. (e.g. \"1,2,3,4,5\" \"6,7,8,9\"")
                    )
                    ])
            .arg(
                Arg::new("verbose")
                        .short('v')
                        .long("verbose")
                        .action(clap::ArgAction::Count)
                        .help("Set the verbosit level. OFF (default), -v => INFO, -vv => DEBUG, -vvv => TRACE"),
        )
        .get_matches()
}
