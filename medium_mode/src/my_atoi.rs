use std::cmp::{max, min};
use std::collections::HashMap;

///
/// refer to https://leetcode.cn/problems/string-to-integer-atoi/solutions/183164/zi-fu-chuan-zhuan-huan-zheng-shu-atoi-by-leetcode-/
/// automation
///
pub fn my_atoi(s: String) -> i32 {
    // state_table: state and its tranform state
    // " " +/- number other
    let mut state_table = HashMap::new();
    state_table.insert("start", vec!["start", "signed","in_number","end"]);
    state_table.insert("signed", vec!["end", "end","in_number","end"]);
    state_table.insert("in_number", vec!["end", "end","in_number","end"]);
    state_table.insert("end", vec!["end", "end","end","end"]);

    let mut res:i64 = 0;
    let mut state = "start";
    let mut sign = 1;
    for c in s.chars() {
        let col = judge_cur_state(c);
        state = state_table.get(state).unwrap()[col];
        if state == "in_number" {
            res = res * 10 + c.to_string().parse::<i64>().unwrap();
            if sign == 1 {
                res = min(res, i32::MAX as i64);
            } else if sign == -1 {
                res = min(res, -(i32::MIN as i64));
            }
        } else if state == "signed" {
            if c == '+' {
                sign = 1;
            } else {
                sign = -1;
            }
        }
    }
    sign * (res as i32)
}

pub fn judge_cur_state(s:char) -> usize {
    if s.is_whitespace() {
        return 0;
    } else if s=='+' || s=='-' {
        return 1;
    } else if s.is_numeric() {
        return 2;
    } else {
        return 3;
    }
}

#[cfg(test)]
mod tests {
    use std::intrinsics::transmute;
    use crate::my_atoi::my_atoi;

    #[test]
    fn test_a_to_i() {
        let mut i = my_atoi("42".to_string());
        println!("{}", i);
        i = my_atoi("   -042".to_string());
        println!("{}", i);
        i = my_atoi("1337c0d3".to_string());
        println!("{}", i);
        i = my_atoi("0-1".to_string());
        println!("{}", i);
        i = my_atoi("words and 987".to_string());
        println!("{}", i);
        // let i = my_atoi("   -042".to_string());
        // println!("{}", i);
    }
}