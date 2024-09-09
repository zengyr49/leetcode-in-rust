
pub fn is_palindrome(mut x:i32) -> bool{
    // negative number
    if x < 0 {
        return false;
    }
    // ends with 0
    if x % 10 == 0 {
        return false;
    }
    let mut reverse_half = 0;
    while x > reverse_half {
        reverse_half = reverse_half * 10 + x%10;
        x = x/10;
    }

    return x == reverse_half || x == reverse_half/10;
}

#[cfg(test)]
mod test {
    use crate::is_palindrome::is_palindrome;

    #[test]
    fn test_palindrome() {
        let mut res;
        res = is_palindrome(123);
        println!("{}", res);
        res = is_palindrome(123321);
        println!("{}", res);
        res = is_palindrome(12321);
        println!("{}", res);
        res = is_palindrome(1233433450);
        println!("{}", res);
        res = is_palindrome(0);
        println!("{}", res);
        res = is_palindrome(-1);
        println!("{}", res);
        res = is_palindrome(1232);
        println!("{}", res);
        res = is_palindrome(1230321);
        println!("{}", res);

    }
}