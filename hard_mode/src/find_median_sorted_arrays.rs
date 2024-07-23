use std::cmp::{max, min};
use std::i32::{MAX, MIN};

///
/// we try to find two separate bars of which left items are smaller than those on the right of the two bars.
/// and the number of left items should be less or equals to the number of items on the right
/// like this
/// 1 2 3 4 5 6| 7 8 9
/// 5 6| 7 8 9 10 11
///
/// notice：the sum of the index of the two bar should be (m+n)/2, where m and n are the lengths of input array
/// that is we should find largest A[i] which is smaller than B[j] where i + j = (m+n)/2
/// assume that bar is on the left like this: (m+n)/2 -1
///
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays(nums2, nums1);
    }

    // find two bars
    let l1 = nums1.len();
    let l2 = nums2.len();
    let mut left = 0;
    let mut right = l1.clone();
    // med1 is the largest number in the left and med2 is the smallest number on the right
    let mut med1 = 0;
    let mut med2 = 0;

    while left <= right {
        let i = (&left + &right) / 2;
        let j = (&l1+&l2+1)/2-i;

        let num_im1 = if i==0 { MIN } else { nums1[i-1] };
        let num_jm1 = if j==0 { MIN } else { nums2[j-1] };
        let num_i = if i==l1 { MAX } else { nums1[i] };
        let num_j = if j==l2 { MAX } else { nums2[j] };

        if num_im1 <= num_j {
            med1 = max(num_im1, num_jm1);
            med2 = min(num_i, num_j);
            left = i + 1;
        } else {
            right = i - 1
        }
    }
    if (l1+l2)%2==0 { ((med1 as f64 + med2 as f64) / 2.0) } else { med1 as f64 }
}

#[cfg(test)]
mod tests {
    use crate::find_median_sorted_arrays::find_median_sorted_arrays;

    #[test]
    fn test_find_mid() {
        let num1 = vec![1,3,4,9];
        let num2 = vec![1,2,3,4,5,6,7,8,9];
        let arrays = find_median_sorted_arrays(num1, num2);
        assert_eq!(arrays, 4.0)
    }
}