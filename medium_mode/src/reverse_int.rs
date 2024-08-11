///
/// key point is to use a proper range in which we can not use long type
/// refer to https://leetcode.cn/problems/reverse-integer/solutions/755611/zheng-shu-fan-zhuan-by-leetcode-solution-bccn/
///
pub fn reverse(x: i32) -> i32 {
    let mut rev = 0;
    let mut mut_x = x;
    while mut_x != 0 {
        if rev < (i32::MIN/10) || rev > (i32::MAX/10) {
            return 0;
        }
        let digit = mut_x%10;
        mut_x = mut_x / 10;
        rev = rev * 10 + digit;
    }

    rev
}

#[cfg(test)]
mod tests {
    use crate::reverse_int::reverse;

    #[test]
    fn test_revers() {
        println!("max i32:{}", i32::MAX);
        let res = reverse(123);
        println!("{}", res);
        assert_eq!(res, 321);

        let res = reverse(0);
        println!("{}", res);
        assert_eq!(res,0);

        let res = reverse(463847412);
        assert_eq!(res,214748364);
    }
}