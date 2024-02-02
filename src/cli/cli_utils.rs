use crate::choose;
use crate::cli::cli;
use crate::cli::cli::Combinatorics;
use crate::generate_number_list;
use crate::linked_list_utils::*;
use crate::math_and_numbers::add_two_nums::Solution as AddNumsSolution;
use crate::math_and_numbers::int_to_roman::Solution as IntToRoman;
use crate::math_and_numbers::median_sorted_arrays::Solution as FindMedian;
use crate::math_and_numbers::merge_two_sorted_lists::Solution as MergeSortedLists;
use crate::math_and_numbers::most_water::Solution as MostWater;
use crate::permutation;
use crate::string_manipulation::valid_parenthesis::Solution as ValidParenthesis;

use ::std::io;
use clap_complete::{generate, shells::Shell};
use num_format::{Locale, ToFormattedString};
use std::collections::linked_list;
use tracing::info;

use clap::ArgMatches;
pub fn parse_list_from_str(s: &str) -> Vec<i32> {
    s.chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}
pub fn handle_add_two_numbers(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let num1 = matches.get_one::<i32>("num1").unwrap();
    let num2 = matches.get_one::<i32>("num2").unwrap();
    let list1: Vec<i32> = num1
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    let list2: Vec<i32> = num2
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let linked_list1 = create_list_from_vec(list1);
    let linked_list2 = create_list_from_vec(list2);
    let res_list = AddNumsSolution::add_two_numbers(linked_list1.clone(), linked_list2.clone());
    let res = list_to_int(res_list.clone());
    let ll1_str = format_list(linked_list1);
    let ll2_str = format_list(linked_list2);
    let res_str = format_list(res_list);

    info!("Add Two Numbers: {} + {} == {}", ll1_str, ll2_str, res_str);
    println!("{}", res);
    Ok(())
}
pub fn handle_find_median_from_sorted_arrays(
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(mut values) = matches.get_many::<String>("arrays") {
        let array1 = values.next().unwrap();
        let array2 = values.next().unwrap();
        let mut list1: Vec<i32> = array1
            .split(',')
            .filter_map(|c| c.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>();
        let mut list2: Vec<i32> = array2
            .split(',')
            .filter_map(|c| c.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>();

        list1.sort();
        list2.sort();

        let list1_str = format!(
            "[{}]",
            list1
                .iter()
                .clone()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        let list2_str = format!(
            "[{}]",
            list2
                .clone()
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );

        let res = FindMedian::find_median_sorted_arrays(list1, list2);
        info!(
            "median_sorted_arrays: arr1: {}, arr2: {}",
            list1_str, list2_str
        );
        println!("{}", res);
    }
    Ok(())
}

pub fn handle_most_water(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let vec_string = matches.get_one::<String>("height-array");
    if let Some(water_string) = vec_string {
        let heights: Vec<i32> = water_string
            .split(',')
            .filter_map(|c| c.trim().parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let res = MostWater::max_area(heights);
        println!("{}", res);
    }

    Ok(())
}

pub fn handle_int_to_roman(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let num = *matches.get_one::<i32>("num").unwrap_or(&0);

    if num > 0 {
        let res = IntToRoman::int_to_roman(num);
        println!("{}", res);
    }

    Ok(())
}

pub fn handle_generate_numbers(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let num = *matches.get_one::<i64>("nums").unwrap_or(&0);
    let output = matches.get_one::<String>("output");

    if num > 0 {
        let output_fp = generate_number_list(num, output.as_deref())?;
        println!("Numbers written to file: {}", output_fp);
    }

    Ok(())
}
pub fn handle_combinatorics(
    combinatoric: Combinatorics,
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    // Logic for "choose" and "permutation" commands
    let n = *matches.get_one::<u128>("n").unwrap_or(&0);
    let k = *matches.get_one::<u128>("k").unwrap_or(&0);
    match combinatoric {
        Combinatorics::Choose => {
            let res = choose(n, k)?;
            if res >= 1000000 {
                println!("{} choose {} = {:e}", n, k, res);
            } else {
                println!(
                    "{} choose {} = {}",
                    n,
                    k,
                    res.to_formatted_string(&Locale::en)
                );
            }
        }
        Combinatorics::Permutation => {
            let res = permutation(n, k)?;
            if res >= 1000000 {
                println!("{} permutation {} = {:e}", n, k, res);
            } else {
                println!(
                    "{} permutation {} = {}",
                    n,
                    k,
                    res.to_formatted_string(&Locale::en)
                );
            }
        }
    }
    Ok(())
}
pub fn handle_valid_parenthesis(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let brackets = matches.get_one::<String>("brackets").unwrap();
    info!("Valid Parenthesis - brackets: {}", &brackets);
    let res = ValidParenthesis::is_valid(brackets.clone());
    info!("Valid parenthesis: {} -> {}", &brackets, res);
    println!("{}", res);
    Ok(())
}
pub fn handle_merge_two_sorted_lists(
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut sorted_lists = matches.get_many::<String>("sorted-arrays").unwrap();
    let array1: String = sorted_lists.next().unwrap().to_string();
    let array2: String = sorted_lists.next().unwrap().to_string();
    let mut list1: Vec<i32> = array1
        .split(',')
        .filter_map(|c| c.trim().parse::<i32>().ok())
        .collect::<Vec<i32>>();
    let mut list2: Vec<i32> = array2
        .split(',')
        .filter_map(|c| c.trim().parse::<i32>().ok())
        .collect::<Vec<i32>>();

    list1.sort();
    list2.sort();

    let list1_str = format!(
        "[{}]",
        list1
            .iter()
            .clone()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    let list2_str = format!(
        "[{}]",
        list2
            .clone()
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    info!("sorted arrays: arr1: {}, arr2: {}", list1_str, list2_str);
    let linked_list1 = ListNode::from_vec(&list1);
    info!("arr1 converted to linked list: {}", &linked_list1);
    let linked_list2 = ListNode::from_vec(&list2);
    info!("arr2 converted to linked list: {}", &linked_list2);

    let linked_list1 = Some(Box::new(ListNode::from_vec(&list1)));
    let linked_list2 = Some(Box::new(ListNode::from_vec(&list2)));
    let res = MergeSortedLists::merge_two_lists(linked_list1, linked_list2);
    println!("{}", res.unwrap());
    Ok(())
}

pub fn handle_shell_completions(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let shell_type = matches.get_one::<String>("shell").unwrap().to_lowercase();
    let mut cli: clap::Command = cli::build_cli();
    let program_name = cli.get_name().to_string();
    println!("program_name: {:?}", &program_name);
    println!("cli: {:?}", &cli);
    println!("shell_type: {:?}", &shell_type);
    match shell_type.as_str() {
        "bash" => generate(Shell::Bash, &mut cli, program_name, &mut io::stdout()),
        "elvish" => generate(Shell::Elvish, &mut cli, program_name, &mut io::stdout()),
        "fish" => generate(Shell::Fish, &mut cli, program_name, &mut io::stdout()),
        "powershell" | "pwsh" => {
            generate(Shell::PowerShell, &mut cli, program_name, &mut io::stdout())
        }
        "zsh" => generate(Shell::Zsh, &mut cli, program_name, &mut io::stdout()),
        _ => unreachable!(),
    }
    Ok(())
}
