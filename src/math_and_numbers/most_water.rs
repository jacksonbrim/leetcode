use colored::Colorize;

use std::fmt;

struct VecWrapper(Vec<i32>);

struct Highlight<'a> {
    vec_wrapper: &'a VecWrapper,
    highlight_start: Option<usize>,
    highlight_final: Option<usize>,
}
impl<'a> Highlight<'_> {
    fn new(vec_wrapper: &'a VecWrapper, start: usize, fin: usize) -> Highlight<'_> {
        Highlight {
            vec_wrapper,
            highlight_start: Some(start),
            highlight_final: Some(fin),
        }
    }

    fn start(&mut self, start: usize) {
        self.highlight_start = Some(start);
    }
    fn fin(&mut self, fin: usize) {
        self.highlight_final = Some(fin);
    }
}

impl<'a> fmt::Debug for Highlight<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for (count, v) in self.vec_wrapper.0.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?;
            }
            let mut what_to_write = || {
                if let Some(idx) = self.highlight_start {
                    if count == idx as usize {
                        return write!(f, "{}", format!("{}", v).green().bold());
                    }
                }
                if let Some(idx) = self.highlight_final {
                    if count == idx as usize {
                        return write!(f, "{}", format!("{}", v).red().bold());
                    }
                }
                write!(f, "{}", v)
            };
            what_to_write()?;
        }
        write!(f, " ]")
    }
}

pub struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 2 {
            return 0;
        }
        let mut left: usize = 0;
        let mut right: usize = n - 1;
        let mut max_area = 0;
        while left < right {
            let current_area: i32 =
                std::cmp::min(height[left], height[right]) * (right - left) as i32;
            if current_area > max_area {
                max_area = current_area;
            }
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    
    
    #[test]
    fn middle_to_end_success() {
        let heights: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let output = 49;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }

    #[test]
    fn length_1_success() {
        let heights: Vec<i32> = vec![1, 1];
        let output = 1;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn middle_heights_greater_than_points_success() {
        let heights: Vec<i32> = vec![1, 2, 1];
        let output = 2;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn max_area_does_not_contain_max_heights_success() {
        // [1,2] -> [3,3]
        // length 2, heights 2, area = 4
        let heights: Vec<i32> = vec![1, 2, 4, 3];
        let output = 4;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn zeros_and_interspersed_heights_success() {
        let heights: Vec<i32> = vec![0, 14, 6, 2, 10, 9, 4, 1, 10, 3];
        let output = 70;
        let res = Solution::max_area(heights);

        assert_eq!(res, output);
    }
    #[test]
    fn scaled_down_list_success() {
        let heights: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 4, 3, 2, 1];
        let output = 12;
        let res = Solution::max_area(heights);
        assert_eq!(res, output);
    }

    //#[test]
    //fn massive_list_success() {
    //    let data = fs::read_to_string("./test_input/most_water.txt")
    //        .expect("Opening file test_input/most_water.txt");
    //    let heights: Vec<i32> = data
    //        .trim()
    //        .split_terminator(",")
    //        .map(|num_str| {
    //            num_str
    //                .parse::<i32>()
    //                .map_err(|e| format!("Failed to parse '{}': {}", num_str, e))
    //        })
    //        .collect::<Result<Vec<i32>, _>>()
    //        .expect("One or more items failed to parse");
    //    println!("big list length: {:?}", heights.len());
    //    let output = 50000000;
    //    let res = Solution::max_area(heights);

    //    assert_eq!(res, output);
    //}
}
