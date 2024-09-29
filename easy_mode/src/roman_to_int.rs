use std::collections::HashMap;
use lazy_static::lazy_static;

struct Solution;


lazy_static! {
    static ref ROMAN_TO_INT_MAP: HashMap<char, i32> = {
        let mut m = HashMap::new();
        m.insert('I', 1);
        m.insert('V', 5);
        m.insert('X', 10);
        m.insert('L', 50);
        m.insert('C', 100);
        m.insert('D', 500);
        m.insert('M', 1000);
        m
    };
}

impl Solution {

    ///
    /// just construct a map and match
    /// but this case should be a good case for static variable and string traversing
    ///
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let n = s.len();
        for (i,x) in s.chars().enumerate() {
            let cur_value = ROMAN_TO_INT_MAP.get(&x).unwrap();
            if i < (n-1) && *cur_value < *ROMAN_TO_INT_MAP.get(&(s.chars().nth(i+1).unwrap())).unwrap() {
                res -= cur_value;
            } else {
                res += cur_value;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use crate::roman_to_int::Solution;

    #[test]
    fn test_roman_to_int() {
        let mut res = Solution::roman_to_int("LVIII".to_string());
        println!("{}", res);
        res = Solution::roman_to_int("MCMXCIV".to_string());
        println!("{}", res)
    }
}