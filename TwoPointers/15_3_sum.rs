impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut sorted_nums: Vec<i32> = nums.clone();
        sorted_nums.sort();

        for (index, elem) in sorted_nums.iter().enumerate() {
            if index > 0 && *elem == sorted_nums[index - 1] {
                continue;
            }

            let mut i = index + 1;
            let mut j = sorted_nums.len() - 1;
            while i < j {
                match (sorted_nums[j] + sorted_nums[i] + *elem).cmp(&0) {
                    cmp::Ordering::Less => {
                        i += 1;
                    }
                    cmp::Ordering::Greater => {
                        j -= 1;
                    }
                    cmp::Ordering::Equal => {
                        res.push(Vec::from([*elem, sorted_nums[i], sorted_nums[j]]));
                        i += 1;
                        while sorted_nums[i] == sorted_nums[i - 1] && i < j {
                            i += 1;
                        }
                    }
                }
            }
        }

        res
    }
}
