pub struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() <= 1 {
            return false;
        }
        let mut open_brackets: Vec<u8> = Vec::new();
        for byte in s.bytes() {
            if Self::is_open_bracket(&byte) {
                open_brackets.push(byte);
            } else {
                if let Some(open_bracket) = open_brackets.pop() {
                    if Self::get_open_bracket(&byte) != open_bracket {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        open_brackets.is_empty()
    }
    fn is_open_bracket(byte: &u8) -> bool {
        match byte {
            b'[' | b'(' | b'{' => true,
            _ => false,
        }
    }

    fn get_open_bracket(byte: &u8) -> u8 {
        match byte {
            b']' => b'[',
            b')' => b'(',
            b'}' => b'{',
            _ => b'0',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn single_parenthesis_true() {
        let s = "()".to_string();
        let output = true;
        let res = Solution::is_valid(s);

        assert_eq!(res, output);
    }
    #[test]
    fn simple_all_brackets_true() {
        let s = "()[]{}".to_string();
        let output = true;
        let res = Solution::is_valid(s);

        assert_eq!(res, output);
    }
    #[test]
    fn simple_false() {
        let s = "(]".to_string();
        let output = false;
        let res = Solution::is_valid(s);

        assert_eq!(res, output);
    }
    #[test]
    fn only_open_brackets_fail() {
        let s = "([({".to_string();
        let output = false;
        let res = Solution::is_valid(s);

        assert_eq!(res, output);
    }
    #[test]
    fn only_one_open_bracket_fail() {
        let s = "(".to_string();
        let output = false;
        let res = Solution::is_valid(s);

        assert_eq!(res, output);
    }
}
