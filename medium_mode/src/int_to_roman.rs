struct Solution;

impl Solution {
    ///
    /// just simulate. we first construct a list ordered by roman number's size
    /// then just minus and check
    ///
    pub fn int_to_roman(num: i32) -> String {
        let nums = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let romans = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
        let mut mut_num = num;
        let mut res_str = "".to_string();
        for i in 0..nums.len() {
            let cur_roman = romans[i];
            let cur_num = nums[i];

            while mut_num >= cur_num {
                res_str.push_str(cur_roman);
                mut_num -= cur_num
            }
        }

        res_str
    }
}


#[cfg(test)]
mod test {
    use crate::int_to_roman::Solution;

    #[test]
    fn test_int_to_roman() {
        let mut res_roman = Solution::int_to_roman(3749);
        println!("{}", res_roman);

        let mut res_roman = Solution::int_to_roman(58);
        println!("{}", res_roman);

        let mut res_roman = Solution::int_to_roman(1994);
        println!("{}", res_roman);
    }
}