// get all pair ranges
// expand pair ranges to 3 and 4 palindromes
// identify palindrome reflections
struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return "".to_string();
        }

        // Transform s into t.
        // For example, s = "abba", t = "^#a#b#b#a#$".
        // ^ and $ signs are sentinels appended to each end to avoid bounds checking
        let t = format!(
            "^#{}#$",
            s.chars().map(|c| format!("#{}", c)).collect::<String>()
        );
        let t_chars: Vec<_> = t.chars().collect();
        let mut p = vec![0; t.len()];
        let mut center = 0;
        let mut right = 0;
        let mut max_len = 0;
        let mut center_index = 0;

        for i in 1..t.len() - 1 {
            let mirror = if i < right { 2 * center - i } else { 0 }; // Safe to use mirror only when i < right

            if i < right {
                p[i] = std::cmp::min(right - i, p[mirror]);
            }

            // Attempt to expand palindrome centered at i
            while i + p[i] + 1 < t.len()
                && i >= p[i] + 1
                && t_chars[i + 1 + p[i]] == t_chars[i - 1 - p[i]]
            {
                p[i] += 1;
            }

            // If palindrome centered at i expands past right,
            // adjust center based on expanded palindrome.
            if i + p[i] > right {
                center = i;
                right = i + p[i];
            }

            if p[i] > max_len {
                // Track max length of palindrome
                max_len = p[i];
                center_index = i;
            }
        }

        // Extract the longest palindrome, strip out the '#'s
        s[(center_index - 1 - max_len) / 2..(center_index - 1 + max_len) / 2].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success1() {
        // "aba" is also a valid answer.
        let input_str1 = "babad".to_string();
        let output1 = "bab".to_string();
        let res1 = Solution::longest_palindrome(input_str1);
        assert_eq!(res1, output1);
    }
    #[test]
    fn success2() {
        let input_str2 = "cbbd".to_string();
        let output2 = "bb".to_string();
        let res2 = Solution::longest_palindrome(input_str2);
        assert_eq!(res2, output2);
    }

    #[test]
    fn success3() {
        let input_str3 = "ajbkdoaadjfjnakjeaaejkanonakeakfannaoeoannaoeakjf".to_string();
        let output3 = "nakjeaaejkan".to_string();
        let res3 = Solution::longest_palindrome(input_str3);
        assert_eq!(res3, output3);
    }
}
