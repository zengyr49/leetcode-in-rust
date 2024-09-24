use std::cmp::{max, min};

struct Solution {
}

///
/// two pointers, one on the left and the other on the right. always move the one points to the shorter one
/// because we need to find higher pipe to find a probable container with larger volume
///
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = (height.len()-1) as i32;
        let mut max_res = 0;
        while left < right {
            let cur_min = min(height[left as usize], height[right as usize]);
            max_res = max(cur_min * (right - left), max_res);
            if height[left as usize] <= height[right as usize] {
                left = left + 1;
            } else {
                right = right - 1;
            }
        }
        max_res
    }
}

#[cfg(test)]
mod tests {
    use crate::max_area::Solution;

    #[test]
    fn test_max_area() {
        let mut res = Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(res, 49);

        res = Solution::max_area(vec![1, 1]);
        assert_eq!(res, 1);
    }

}