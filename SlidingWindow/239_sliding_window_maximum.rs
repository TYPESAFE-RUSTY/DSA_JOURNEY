impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut max = i32::MIN;

        if k == 0 {
            return res;
        }

        let mut window = nums.windows(k as usize);

        if let Some(items) = window.next() {
            for item in items {
                max = max.max(*item);
            }
            res.push(max);
        }

        while let Some(items) = window.next() {
            if items.contains(&max) {
                max = max.max(*items.last().unwrap());
            } else {
                max = i32::MIN;
                for item in items {
                    max = max.max(*item);
                }
            }

            res.push(max)
        }

        res
    }
}
