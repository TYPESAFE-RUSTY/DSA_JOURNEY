use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len();
        let mut mid: usize;
        while low < high {
            mid = (high + low) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Less => {
                    high = mid;
                }
                Ordering::Greater => {
                    low = mid + 1;
                }
            }
        }
        -1
    }
}
