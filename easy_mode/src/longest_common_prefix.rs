use std::convert::identity;

struct Solution;

impl Solution {
    pub fn longest_common_prefix_violence(strs: Vec<String>) -> String {
        if strs.get(0).unwrap().eq("") {
            return "".to_string();
        }
        let mut cur_str = strs.get(0).unwrap().chars().nth(0).unwrap();
        let mut idx = 0;
        let mut go_on = true;
        let mut res = "".to_string();

        while go_on {
            for x in &strs {
                if idx < x.len() {
                    let temp_str = x.chars().nth(idx).unwrap();
                    if !temp_str.eq(&cur_str) {
                        go_on = false;
                        break;
                    }
                } else {
                    go_on = false;
                    break;
                }
            }
            if go_on {
                res.push_str(&cur_str.to_string());
                idx += 1;
                if idx < strs.get(0).unwrap().len() {
                    cur_str = strs.get(0).unwrap().chars().nth(idx).unwrap();
                }
            }
        }
        res
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut prefix = strs[0].clone();
        for x in strs.iter().skip(1) {
            let mut temp = String::new();
            for (c1, c2) in prefix.chars().zip(x.chars()) {
                if c1==c2 {
                    // longest common prefix between previous prefix and current word
                    temp.push(c1);
                } else {
                    break;
                }
            }
            prefix = temp;
            // jump out immediately
            if prefix.is_empty() {
                return "".to_string();
            }
        }

        prefix
    }


}

#[cfg(test)]
mod test {
    use crate::longest_common_prefix::Solution;

    #[test]
    pub fn test() {
        // let ininput = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
        let ininput = vec!["do".to_string(), "d".to_string(), "dosss".to_string()];
        let res = Solution::longest_common_prefix(ininput);
        println!("{}", res)
    }
}