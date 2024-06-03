use std::cmp::Ordering;

impl Solution {
    pub fn find_min_my_version(nums: Vec<i32>) -> i32 {
        let mut temp: i32 = nums[0];
        let mut low: usize = 0;
        let mut high: usize = nums.len() - 1;

        while low < high {
            let mid = (low + high) / 2;
            if mid < high && nums[mid + 1] < nums[mid] {
                return nums[mid + 1];
            }
            if mid > low && nums[mid - 1] < nums[mid] {
                temp = i32::min(temp, nums[mid]);
                if temp <= nums[mid] {
                    high = mid;
                } else {
                    low = mid + 1;
                }
            } else {
                return nums[mid];
            }
        }
        temp
    }

    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut res: i32 = nums[0];
        let mut low: usize = 0;
        let mut high: usize = nums.len() - 1;

        while low < high {
            if nums[low] <= nums[high] {
                res = i32::min(res, nums[low]);
                break;
            }

            let mid = (low + high) / 2;
            res = i32::min(res, nums[mid]);
            match nums[mid].cmp(&nums[low]) {
                Ordering::Less => {
                    high = mid;
                }
                _ => {
                    low = mid + 1;
                }
            }
        }
    }
}
