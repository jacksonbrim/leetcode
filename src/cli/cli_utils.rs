use crate::choose;
use crate::generate_number_list;
use crate::math_and_numbers::add_two_nums::Solution as AddNumsSolution;
use crate::math_and_numbers::int_to_roman::Solution as IntToRoman;
use crate::math_and_numbers::median_sorted_arrays::Solution as FindMedian;
use crate::permutation;
use num_format::{Locale, ToFormattedString};
use tracing::info;

use clap::ArgMatches;
pub fn parse_list_from_str(s: &str) -> Vec<i32> {
    s.chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>()
}
pub fn handle_add_two_numbers(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(mut values) = matches.get_many::<i32>("add-two-numbers") {
        let num1 = *values.next().unwrap();
        let num2 = *values.next().unwrap();
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

        let linked_list1 = AddNumsSolution::create_list_from_vec(list1);
        let linked_list2 = AddNumsSolution::create_list_from_vec(list2);
        let res_list = AddNumsSolution::add_two_numbers(linked_list1.clone(), linked_list2.clone());
        let res = AddNumsSolution::list_to_int(res_list.clone());
        let ll1_str = AddNumsSolution::format_list(linked_list1);
        let ll2_str = AddNumsSolution::format_list(linked_list2);
        let res_str = AddNumsSolution::format_list(res_list);

        info!("Add Two Numbers: {} + {} == {}", ll1_str, ll2_str, res_str);
        println!("{}", res);
    }
    Ok(())
}
pub fn handle_find_median_from_sorted_arrays(
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(mut values) = matches.get_many::<String>("find-median-sorted-arrays") {
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

pub fn handle_int_to_roman(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let num = *matches.get_one::<i32>("int-to-roman").unwrap_or(&0);

    if num > 0 {
        let res = IntToRoman::int_to_roman(num);
        println!("{}", res);
    }

    Ok(())
}

pub fn handle_generate_numbers(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let num = *matches.get_one::<i64>("generate-numbers").unwrap_or(&0);
    let output = matches.get_one::<String>("output");

    if num > 0 {
        let output_fp = generate_number_list(num, output.as_deref())?;
        println!("Numbers written to file: {}", output_fp);
    }

    Ok(())
}
pub fn handle_combinatorics(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    // Logic for "choose" and "permutation" commands
    if let Some(mut values) = matches.get_many::<u128>("choose") {
        let n = *values.next().unwrap();
        let k = *values.next().unwrap();

        let res = choose(n as u128, k as u128)?;
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
    if let Some(mut values) = matches.get_many::<u128>("permutation") {
        let n = *values.next().unwrap();
        let k = *values.next().unwrap();

        let res = permutation(n as u128, k as u128)?;
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
    Ok(())
}
