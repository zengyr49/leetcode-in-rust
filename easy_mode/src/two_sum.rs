
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // save all numbers and their index in a map
    let mut map = HashMap::new();
    let mut result = Vec::new();
    for (i, num) in nums.iter().enumerate() {
        map.insert(num, i);
    }
    // map and get
    for (i, &num) in nums.iter().enumerate() {
        let comp = target - &num;
        if let Some(&prob_index) = map.get(&comp) {
            if prob_index == i {
                continue;
            }
            result.push(i as i32);
            result.push(prob_index as i32);
            return result;
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::two_sum::two_sum;

    #[test]
    fn test_two_sum() {
        let nums = vec![3,2,4];
        let target = 6;
        let res = two_sum(nums, target);

        println!("the res is [{},{}]", res.get(0).unwrap(), res.get(1).unwrap());

        assert!(res.contains(&(2 as i32)));
    }
}