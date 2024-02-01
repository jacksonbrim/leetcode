use clap::{command, Arg, ArgAction, ArgMatches, Command};

pub fn parse_args() -> ArgMatches {
    Command::new("Leetcode Solutions")
        .author("Jackson Brim")
        .version("0.1.0")
        .about("A program for running leetcode solutions")
        .arg(
            Arg::new("generate-numbers")
                .short('g')
                .long("generate-numbers")
                .num_args(1)
                .value_parser(clap::value_parser!(i64))
                .help("Generate numbers and output to file (if specified with --output)"),
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
        .arg(
            Arg::new("choose")
                .long("choose")
                .value_names(&["n", "k"])
                .num_args(2)
                .value_parser(clap::value_parser!(u128))
                .required(false)
                .help("Calculates combinations, requires two numbers where n > k and both > 0. In a combinatorics, n choose k represents a combination of n things taken k at a time without repetition"),
        )
        .arg(
            Arg::new("permutation")
                .long("permutation")
                .value_names(&["n", "k"])
                .num_args(2)
                .value_parser(clap::value_parser!(u128))
                .required(false)
                .help("Calculates permutations, requires two numbers where n > k and both > 0"),
        )
        .arg(
            Arg::new("add-two-numbers")
                .long("add-two-numbers")
                .value_names(&["num1", "num2"])
                .num_args(2)
                .value_parser(clap::value_parser!(i32))
                .help("Given two non-empty linked lists representing two non-negative integers stored in reverse order. Each of their nodes contians a single digit. Add the two numbers return the sum as a linked list."),
        )

        .get_matches()
}
