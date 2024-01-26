struct Solution;
impl Solution {
    // 1ms, beats 77.45% of users with rust, 2.12MiB beats 61.70% of rust users
    pub fn longest_common_prefix_1(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs[0].clone();
        } else if strs.len() == 0 {
            return String::with_capacity(0);
        }

        let mut longest_prefix: String = String::new();
        // iterate over words, stopping short of the last element
        'outer_loop: for idx in 0..(strs.len() - 1) {
            // zip for easy comparison
            //
            let mut this_prefix = String::new();
            for (word_a, word_b) in std::iter::zip(strs[idx].chars(), strs[idx + 1].chars()) {
                if idx != 0 {
                    if longest_prefix.is_empty() {
                        return longest_prefix.to_string();
                    }
                }

                if word_a == word_b {
                    this_prefix.push(word_a);
                } else {
                    if this_prefix.len() < longest_prefix.len() || idx == 0 {
                        longest_prefix = this_prefix;
                    }
                    continue 'outer_loop;
                }
            }
            if this_prefix.len() < longest_prefix.len() || idx == 0 {
                longest_prefix = this_prefix;
            }
        }
        longest_prefix.to_string()
    }
    // trying again with an interspered method
    // 1ms, beats 77.45% of users w/ rust, 2.15MB beats 61.70% of rust users
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let strs_len = strs.len();
        // Check if any string is empty or if the input is empty
        if strs.is_empty() || strs.iter().any(|s| s.is_empty()) {
            return String::new();
        }
        // If there's only one string, return it
        if strs_len == 1 {
            return strs[0].clone();
        }
        let interspersed_strs: Vec<u8> = Self::intersperse_strings(strs);
        let mut common_prefixes: String = String::new();
        'chunk_loop: for chunk in interspersed_strs.chunks(strs_len) {
            if let Some(&first_ch) = chunk.first() {
                let current_char = first_ch as char; // Convert u8 to char
                if chunk.iter().len() < strs_len {
                    break 'chunk_loop;
                }
                if chunk.iter().all(|&x| x as char == current_char) {
                    common_prefixes.push(current_char);
                } else {
                    break 'chunk_loop;
                }
            }
        }
        common_prefixes
    }
    pub fn longest_common_prefix_(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        // Find the shortest string in the array
        let shortest_string = strs.iter().min_by_key(|s| s.len()).unwrap();
        if shortest_string.is_empty() {
            return String::new();
        }
        let mut common_prefix = String::new();
        for (idx, ch) in shortest_string.chars().enumerate() {
            // Check if this character is present at the same position in all strings
            if strs.iter().all(|s| s.as_bytes()[idx] == ch as u8) {
                common_prefix.push(ch);
            } else {
                break;
            }
        }
        common_prefix
    }
    fn intersperse_strings(strs: Vec<String>) -> Vec<u8> {
        let mut iterators: Vec<_> = strs.iter().map(|s| s.bytes()).collect();
        let min_length = strs.iter().map(|s| s.len()).min().unwrap_or(0);
        let mut result = Vec::new();

        for _ in 0..min_length {
            for it in iterators.iter_mut() {
                if let Some(byte) = it.next() {
                    result.push(byte);
                }
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_three_success() {
        let strs = vec!["flower", "flow", "flight"];
        let strs: Vec<String> = strs.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let output = String::from("fl");
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, output);
    }
    #[test]
    fn only_two_matches_fail() {
        let strs = vec!["dog", "racecar", "car"];
        let strs: Vec<String> = strs.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        let output = String::new();
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, output);
    }
    #[test]
    fn two_strs_fail() {
        let strs = vec![String::new(), String::from("b")];
        let output = String::new();
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, output);
    }
    #[test]
    fn two_strs_success() {
        let strs = vec![String::from("ab"), String::from("a")];
        let output = String::from("a");
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, output);
    }
    #[test]
    fn three_strs_success() {
        let strs = vec![String::from("aaa"), String::from("aa"), String::from("aaa")];
        let output = String::from("aa");
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, output);
    }
    #[test]
    fn another_two_str_success() {
        let strs = vec![String::from("ab"), String::from("abcc")];
        let output = String::from("ab");
        let res = Solution::longest_common_prefix(strs);
        assert_eq!(res, output);
    }
}
