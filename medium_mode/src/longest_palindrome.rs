// 示例 1：
//
// 输入：s = "babad"
// 输出："bab"
// 解释："aba" 同样是符合题意的答案。
// 示例 2：
//
// 输入：s = "cbbd"
// 输出："bb"

use std::cmp::max;

///
/// expand from center
/// two situations: xax  xaax
///
///
pub fn longest_palindrome(s: String) -> String {
    // border
    if s.len() == 0 || s.len() == 1 {
        return s;
    }
    let ls = s.len();
    // println!("this is {}", &s[1..2]);

    let mut res_len = 0;
    let mut res_left = 0;
    for i in 0..ls {
        let len1 = expandMethod(&s, i as i32, i as i32);
        let len2 = expandMethod(&s, i as i32, (i+1)  as i32);
        let cur_max_len = max(len1, len2);
        if (cur_max_len > res_len) {
            res_len = cur_max_len;
            res_left = i - (res_len-1)/2;
        }
    }
    s[res_left..res_left+res_len].to_string()
}

///
/// s origin string
/// left index of center string
/// right index of center string
fn expandMethod(s:&String, mut left:i32, mut right:i32) -> usize{
    while left >=0 && right < s.len() as i32 && &s[left as usize..(left+1) as usize] == &s[right as usize..(right+1) as usize] {
        left-=1;
        right+=1;
    }
    (right - left - 1) as usize
}

#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring::length_of_longest_substring;
    use crate::longest_palindrome::longest_palindrome;

    #[test]
    pub fn test_longest_palindrome() {
        let s = "abacdaeaddae".to_string();
        let string = longest_palindrome(s);
        assert_eq!(string, "eaddae".to_string())
    }
}