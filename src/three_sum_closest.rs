#[derive(Debug)]
pub struct ClosestSum {
    distance: i32,
    sum: i32,
}
impl ClosestSum {
    fn new() -> Self {
        Self {
            distance: 0,
            sum: 0,
        }
    }
    fn from(distance: i32, sum: i32) -> Self {
        Self { distance, sum }
    }
}
struct Solution;
impl Solution {
    // 3ms, beats 98.95%, 2.16MB Memory, Beats 87.37%
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();

        let mut closest = nums[0] + nums[1] + nums[n - 1];
        for i in 0..(n - 2) {
            // skip the repeats
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;
            while left < right {
                println!("[{:?}, {:?}, {:?}]", nums[i], nums[left], nums[right]);
                let sum = nums[i] + nums[left] + nums[right];

                if (sum - target).abs() < (closest - target).abs() {
                    closest = sum;
                }

                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    return target;
                }
            }
        }
        closest
    }

    // 9ms, beats 58.95%, 2.18MB Memory, Beats 45.25%
    pub fn first_attempt_three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();

        let mut closest_sum: Option<ClosestSum> = None;
        for i in 0..n {
            // skip the repeats
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                let new_distance = (sum - target).abs();
                if sum == target {
                    return sum;
                }
                if let Some(closest) = &closest_sum {
                    if new_distance < closest.distance {
                        let new_closest = ClosestSum::from(new_distance, sum);
                        closest_sum = Some(new_closest);
                    }
                } else {
                    let new_closest = ClosestSum::from(new_distance, sum);
                    closest_sum = Some(new_closest);
                }
                while left > right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left > right && nums[right] == nums[right + 1] {
                    right -= 1;
                }
                if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        if let Some(result) = closest_sum {
            return result.sum;
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let output = 2;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, output);
    }

    #[test]
    fn success2() {
        let nums = vec![-1, 2, 1, -4];
        let target = 1;
        let output = 2;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, output);
    }
    #[test]
    fn all_negatives_success() {
        let nums = vec![-1000, -5, -5, -5, -5, -5, -5, -1, -1, -1];
        let target = -14;
        let output = -15;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, output);
    }
    #[test]
    fn big_list_success() {
        let nums = vec![
            -43, 57, -71, 47, 3, 30, -85, 6, 60, -59, 0, -46, -40, -73, 53, 68, -82, -54, 88, 73,
            20, -89, -22, 39, 55, -26, 95, -87, -57, -86, 28, -37, 43, -27, -24, -88, -35, 82, -3,
            39, -85, -46, 37, 45, -24, 35, -49, -27, -96, 89, 87, -62, 85, -44, 64, 78, 14, 59,
            -55, -10, 0, 98, 50, -75, 11, 97, -72, 85, -68, -76, 44, -12, 76, 76, 8, -75, -64, -57,
            29, -24, 27, -3, -45, -87, 48, 10, -13, 17, 94, -85, 11, -42, -98, 89, 97, -66, 66, 88,
            -89, 90, -68, -62, -21, 2, 37, -15, -13, -24, -23, 3, -58, -9, -71, 0, 37, -28, 22, 52,
            -34, 24, -8, -20, 29, -98, 55, 4, 36, -3, -9, 98, -26, 17, 82, 23, 56, 54, 53, 51, -50,
            0, -15, -50, 84, -90, 90, 72, -46, -96, -56, -76, -32, -8, -69, -32, -41, -56, 69, -40,
            -25, -44, 49, -62, 36, -55, 41, 36, -60, 90, 37, 13, 87, 66, -40, 40, -35, -11, 31,
            -45, -62, 92, 96, 8, -4, -50, 87, -17, -64, 95, -89, 68, -51, -40, -85, 15, 50, -15, 0,
            -67, -55, 45, 11, -80, -45, -10, -8, 90, -23, -41, 80, 19, 29, 7,
        ];
        let target = 255;
        let output = 255;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, output);
    }
    #[test]
    fn duplicates_success() {
        let nums = vec![1, 1, 1, 0];
        let target = 1;
        let output = 2;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, output);
    }
    #[test]
    fn big_nums_big_list_success() {
        let nums = vec![
            193, -665, -927, -275, 76, 862, -925, -866, 424, -338, -603, 704, -182, -393, 749,
            -707, 894, 599, 514, 916, 641, 654, -376, -198, -26, 181, 834, 626, -762, -417, -244,
            299, 664, -595, -144, 806, 970, 93, -204, 549, 275, 688, 257, 558, -426, 480, 336, 342,
            -337, 39, 992, -648, -343, -609, 633, 709, 400, -822, 71, -372, -622, 125, -626, 855,
            356, -276, -533, 685, -407, -770, -670, 292, 497, 348, -152, 953, -978, -755, 581,
            -252, -491, -392, -610, -211, -20, 891, -51, 439, -945, 982, 241, 469, -530, 842, 883,
            802, -131, 986, 845, -842, 154, 980, 991, 235, -519, 605, 584, -817, -552, -136, -727,
            448, 938, 379, -464, 878, 296, 118, -653, -860, -157, -93, 395, 632, 0, 142, -973, 561,
            17, -645, 655, -336, -290, 11, 823, 677, -497, 438, 890, 947, 927, 434, 799, 707, -940,
            -765, 97, 115, -549, -513, -339, 140, 35, -594, 146, -451, -754, -333, 716, -280, 449,
            172, -803, -135, 564, -879, 912, 451, 618, -112, 200, 797, 874, -592, 588, -559, 881,
            -967, -398, 178, 879, 141, -306, -616, 108, 231, -411, -53, 691, 936, 406, -115, -165,
            134, 368, -571, 85, -138, -346, -412, -266, 364, 391, 27, 629, -589, -210, -539, 979,
            -147, 804,
        ];
        let target = 7991;
        let output = 2969;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, output);
    }
    #[test]
    fn duplicates_success_2() {
        let nums = vec![-2, -1, 1, 4];
        let target = 0;
        let output = 1;
        let res = Solution::three_sum_closest(nums, target);
        assert_eq!(res, output);
    }
}
