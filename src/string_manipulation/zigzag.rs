//middle_row = (i+1) % 2 == 0;
//bottom_row = (i+2) % 4 == 0;
//0, 4, 8, 12
//1, 3, 5, 7, 9, 11, 13
//2, 6, 10,

//0, 10, 20, 30, 40  -> every 10
//1, 9, 11, 19, 21, 29, 31, 39, 41 -> every 8, then 2
//2, 8, 12, 18, 22, 28, 32, 38, 42 -> every 6, then 4
//3, 7, 13, 17, 23, 27, 31, 37, 43 -> every 4, then 6
//4, 6, 14, 16, 24, 26, 32, 34, 42 -> every 2, then 8
//... -> every 10
//PRIYN
//A   II   LS   AP   IG
//Y  H N  A H  P A  R
//P S  G P  I G  L I
//AI   PY   RN   IH
//LAIS

struct Solution;
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let n_rows = num_rows as usize;
        if n_rows <= 1 || s.len() <= n_rows {
            return s.to_string();
        }

        let chars: Vec<char> = s.chars().collect();
        let mut result = String::new();
        let divisor = n_rows * 2 - 2; // Pattern repeats every `divisor` characters

        for row in 0..n_rows {
            let mut i = row;
            while i < chars.len() {
                // Always add the character at the current position
                result.push(chars[i]);

                // If not in the first or last row
                if row > 0 && row < n_rows - 1 {
                    // Calculate the index of the 'zigzag' character
                    let zigzag_index = i + divisor - 2 * row;
                    if zigzag_index < chars.len() {
                        result.push(chars[zigzag_index]);
                    }
                }

                // Move to the next block in the pattern
                i += divisor;
            }
        }

        result.to_string()
    }
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success1() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 3;
        let output = "PAHNAPLSIIGYIR".to_string();
        let res = Solution::convert(s, num_rows);
        assert_eq!(res, output);
    }
    #[test]
    fn success2() {
        let s = "PAYPALISHIRING".to_string();
        let num_rows = 4;
        let output = "PINALSIGYAHRPI".to_string();
        let res = Solution::convert(s, num_rows);
        assert_eq!(res, output);
    }
    #[test]
    fn success3() {
        let s = "A".to_string();
        let num_rows = 1;
        let output = "A".to_string();
        let res = Solution::convert(s, num_rows);
        assert_eq!(res, output);
    }
    #[test]
    fn success4() {
        let s = "PAYPALISHIRINGPAYPALISHIRINGPAYPALISHIRING".to_string();
        let num_rows = 6;
        let output = "PRIYNAIILSAPIGYHNAHPARPSGPIGLIAIPYRNIHLAIS".to_string();
        let res = Solution::convert(s, num_rows);
        assert_eq!(res, output);
    }
}
