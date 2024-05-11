impl Solution {
    pub fn min(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut low: usize = 0;
        let mut high = height.len() - 1;
        let mut max: i32 = 0;
        while low < high {
            let min = Solution::min(height[low], height[high]);
            let temp = min * (high - low) as i32;
            // check if current is more
            if temp > max {
                max = temp;
            }

            // move lower one
            if min == height[low] {
                low += 1;
            } else {
                high -= 1;
            }
        }
        max
    }
}
