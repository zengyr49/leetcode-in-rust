///
/// dynamic programing
/// consider when p[i]=* and p[i] !=*
///

struct Solution { }

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();

        // construct dp
        let mut dp = vec![vec![false; n+1]; m+1];
        dp[0][0] = true;
        for i in 0..m+1 {
            for j in 1..n+1 {
                if p.get(j-1..j).unwrap() == "*" {
                    if Solution::matches(&s, &p, i, j-1) {
                        // if matches, then dp[i][j] can be deduced by dp[i-1][j] or dp[i][j-2]
                        // dp[i-1][j] means p[j-1] （matches/or not） s[i-1], s[i-2]... at least once
                        // dp[i][j-2] means current x* pair does not match
                        dp[i][j] = dp[i-1][j] || dp[i][j-2];
                    } else {
                        dp[i][j] = dp[i][j-2];
                    }
                } else {
                    if Solution::matches(&s, &p, i, j) {
                        dp[i][j] = dp[i-1][j-1]
                    }
                }
            }
        }

        return dp[m][n];
    }

    ///
    /// check matches for s[i] and p[j]
    ///
    fn matches(s:&str, p:&str, i:usize, j:usize) -> bool {
        if i==0 {
            return false;
        }
        // when p[i] is '.', return true
        if p.get(j-1..j).unwrap() == "." {
            return true;
        }
        let x = p.get(j-1..j).unwrap();

        return s.get(i-1..i).unwrap() == p.get(j-1..j).unwrap();
    }
}


#[cfg(test)]
mod tests {

    use crate::is_match::Solution;
    #[test]
    fn test_is_match() {
        let s = "aabbcde";
        let p = "a*bbc.*";
        let res = Solution::is_match(s.to_string(), p.to_string());
        assert_eq!(res, true);
    }

}