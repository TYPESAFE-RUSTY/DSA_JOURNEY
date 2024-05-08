use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut output: Vec<i32> = Vec::new();
        for (index, item) in nums.iter().enumerate() {
            let required = target - item;
            match map.get(&required) {
                Some(i) => {
                    output.push(*i);
                    output.push(index as i32);
                }
                None => {
                    map.insert(*item, index as i32);
                }
            }
        }
        output
    }
}
