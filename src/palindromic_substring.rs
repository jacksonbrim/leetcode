// get all pair ranges
// expand pair ranges to 3 and 4 palindromes
// identify palindrome reflections
pub fn longest_palindrome(s: String) -> String {
    let mut longest_palindrome = String::new();
    let len = s.len();

    // Function to check if the given slice is a palindrome
    fn is_palindrome(slice: &str) -> bool {
        slice.chars().eq(slice.chars().rev())
    }

    for chunk_size in (1..=len).rev() {
        for start in 0..=len - chunk_size {
            let end = start + chunk_size;
            let substring = &s[start..end];

            if is_palindrome(substring) {
                // Check if the substring is a palindrome
                longest_palindrome = substring.to_string();
                return longest_palindrome; // Return immediately as this is the largest possible palindrome for the current chunk_size
            }
        }
    }

    longest_palindrome
}

#[cfg(test)]
mod tests {
    use crate::palindromic_substring::*;
    #[test]
    fn success1() {
        // "aba" is also a valid answer.
        let input_str1 = "babad".to_string();
        let output1 = "bab".to_string();
        let res1 = longest_palindrome(input_str1);
        assert_eq!(res1, output1);
    }
    #[test]
    fn success2() {
        let input_str2 = "cbbd".to_string();
        let output2 = "bb".to_string();
        let res2 = longest_palindrome(input_str2);
        assert_eq!(res2, output2);
    }
}
