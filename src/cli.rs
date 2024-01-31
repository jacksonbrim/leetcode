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
        .arg(
            Arg::new("choose")
                .long("choose")
                .value_names(&["n", "k"])
                .num_args(2)
                .value_parser(clap::value_parser!(u128))
                .required(false)
                .help("Calculates combinations, requires two numbers where n > k and both > 0"),
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
        .get_matches()
}
