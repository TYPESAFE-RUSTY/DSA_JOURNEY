use std::cmp::{self, max};
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        match height.len().cmp(&2) {
            cmp::Ordering::Less => 0,
            _ => {
                let mut low = 0;
                let mut high = height.len() - 1;
                let mut area = 0;
                let mut left_max = height[low];
                let mut right_max = height[high];

                while low < high {
                    if left_max < right_max {
                        low += 1;
                        left_max = max(height[low], left_max);
                        area += left_max - height[low];
                    } else {
                        high -= 1;
                        right_max = max(height[high], right_max);
                        area += right_max - height[high];
                    }
                }

                area
            }
        }
    }
}
