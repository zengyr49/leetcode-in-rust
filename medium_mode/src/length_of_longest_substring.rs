use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    // result
    let mut res = 0;

    // a map to record current window's elements
    let mut item_idx_map = HashMap::new();

    // mark start idx
    let mut outer_start = 0;
    let len_str = s.len();
    if len_str==0 {
        return res;
    }

    while outer_start < len_str {
        let mut cur_max = 0;
        let mut cur_start = outer_start.clone();
        while cur_start<len_str && (!item_idx_map.contains_key(&s[cur_start..cur_start+1])) {
            cur_max += 1;
            item_idx_map.insert(&s[cur_start..cur_start+1], 1);
            cur_start += 1;
        }
        res = if res<cur_max { cur_max } else { res };

        outer_start += 1;
        item_idx_map.clear();
    }
    res
}


#[cfg(test)]
mod tests {
    use crate::length_of_longest_substring::length_of_longest_substring;

    #[test]
    pub fn test_longest_substring() {
        let mut res = length_of_longest_substring("aabcddecfqs".to_string());
        assert_eq!(res, 6);
        res = length_of_longest_substring("a".to_string());
        assert_eq!(res, 1);
        res = length_of_longest_substring("".to_string());
        assert_eq!(res, 0);
    }
}