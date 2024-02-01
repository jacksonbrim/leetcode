# Rust LeetCode Solutions CLI ***WIP!***
## Introduction
Welcome to the Rust LeetCode Solutions CLI! This command-line interface (CLI) application, built with Rust and Clap, offers a convenient way to run and test a variety of LeetCode problem solutions. The application encompasses solutions from various categories such as data structures, dynamic programming, mathematical problems, and string manipulations.

#### Available CLI Commands
Here is a list of commands currently available in the CLI application, along with their descriptions:

1.**Generate Numbers**

2.**Output File Specification**

3.**Verbosity Level**

4.**Combinations (Choose)**
* Command: --choose
* Description: Calculates combinations, requires two numbers where n > k and both > 0. In combinatorics, n choose k represents a combination of n things taken k at a time without repetition.
* Usage: --choose <n> <k> where <n> and <k> are the numbers for combination.

5.**Permutations**
* Command: --permutation
* Description: Calculates permutations, requires two numbers where n > k and both > 0.
* Usage: --permutation <n> <k> where <n> and <k> are the numbers for permutation.

6.**Add Two Numbers**
* Command: --add-two-numbers
* Description: Given two non-empty linked lists representing two non-negative integers stored in reverse order. Each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
* Usage: --add-two-numbers <num1> <num2> where <num1> and <num2> are the numbers.

7.**Integer to Roman Numeral Conversion**
* Command: --int-to-roman
* Description: Integer to Roman Numeral Conversion.
* Usage: --int-to-roman <NUMBER> where <NUMBER> is the integer to convert.

8.**Find Median of Sorted Arrays**
* Command: --median-sorted-arrays
* Description: Find the median from two sorted arrays. Arrays must be comma-separated and surrounded in double quotes (e.g., "1,2,3,4,5" "6,7,8,9").
* Usage: --median-sorted-arrays <ARRAY1> <ARRAY2> where <ARRAY1> and <ARRAY2> are the arrays.

9.**Maximum Area**
* Command: --max-area
* Description: Find the maximum area between any two heights from a list of heights. The list must be comma-separated and surrounded in double quotes (e.g., "1,2,3,4,5" "6,7,8,9").
* Usage: --max-area <HEIGHTS> where <HEIGHTS> is the list of heights.

### Features
* **Wide Range of Solutions**: Solutions to various LeetCode problems including popular ones like "Two Sum", "Median of Two Sorted Arrays", "Longest Substring Without Repeating Characters", and many more.
* **Test Data Generation**: Ability to generate test data for problems, making it easy to try out different scenarios and inputs.
* **Dynamic Programming & Math Solutions**: Includes complex problems solved using dynamic programming and various mathematical approaches.
* **String Manipulation**: Solutions for problems that require string processing and manipulation techniques.
* **Data Structures**: Implements and solves problems using fundamental data structures.
* **Command-Line Flexibility**: Run solutions and tests directly from the command line with easy-to-use commands.
### Subdirectories
* **cli**: Contains cli.rs and cli_utils.rs for CLI-related operations and utilities.
* **data_structures**: Implements common data structures and includes solutions like three_sum.rs and letter_combinations_phone_num.rs.
* **dynamic_programming**: Contains dynamic programming solutions like combinatorics.rs.
* **math_and_numbers**: Houses mathematical and numerical problem solutions such as add_two_nums.rs and roman_to_int.rs.
* **string_manipulation**: Features solutions for string processing problems like longest_common_prefix.rs and zigzag.rs.
### Usage
To run a specific LeetCode solution:

Clone the repository to your local machine.
Navigate to the project directory.
Compile the project using Rust's Cargo tool: cargo build --release.
Run the program using cargo run, followed by specific commands and arguments for the problem you want to solve.
For example:

```bash
cargo run -- --median-sorted-arrays "1,2,3,4,5" "7,8,9"
```
This command runs the "find-median-sorted-arrays" solution with the input array [1,2,3,4, 5], and [7,8,9].

This Rust LeetCode Solutions CLI is open-sourced under the MIT License. See the LICENSE file for more details.
