impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low: usize = 0;
        let mut high = nums.len() - 1;

        while low <= high {
            let mid = (low + high) / 2;
            if target == nums[mid] {
                return mid as i32;
            }

            if nums[low] <= nums[mid] {
                if target > nums[mid] || target < nums[low] {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            } else {
                if target < nums[mid] || target > nums[high] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            }
        }
        -1
    }
}
